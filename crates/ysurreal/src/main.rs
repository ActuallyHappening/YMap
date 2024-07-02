use clap::{Parser, Subcommand};
use tracing::*;

pub mod prelude {
	pub(crate) use camino::Utf8PathBuf;
	pub(crate) use clap::Subcommand;
	pub(crate) use color_eyre::eyre::WrapErr;
	pub(crate) use tracing::*;
	pub(crate) use which::which;
}


#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
	#[clap(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
	/// Executes commands on the server, see [ProductionCommand]
	Production {
		#[clap(flatten)]
		ssh_server: ysurreal::server::SSHServerConnection,

		#[clap(subcommand)]
		production_command: production::ProductionCommand,
	},

	Testing {
		#[clap(subcommand)]
		testing_command: testing::TestingCommand,
	},
}

fn install_tracing() {
	use tracing_error::ErrorLayer;
	use tracing_subscriber::prelude::*;
	use tracing_subscriber::{fmt, EnvFilter};

	let fmt_layer = fmt::layer().with_target(false);
	let filter_layer = EnvFilter::try_from_default_env()
		.or_else(|_| EnvFilter::try_new("info,ysurreal=trace"))
		.unwrap();

	tracing_subscriber::registry()
		.with(filter_layer)
		.with(fmt_layer)
		.with(ErrorLayer::default())
		.init();
}

#[tokio::main]
async fn main() {
	color_eyre::install().expect("Failed to install color_eyre");

	install_tracing();

	let cli = Cli::parse();

	info!("Starting ysurreal CLI");

	match run(cli).await {
		Ok(_) => info!("ysurreal CLI completed successfully"),
		Err(err) => {
			eprintln!("{}", err);
		}
	}
}

async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	match cli.command {
		Commands::Testing { testing_command } => testing::handle(testing_command)?,
		Commands::Production {
			ssh_server,
			production_command,
		} => {
			production::handle(ssh_server, production_command).await?;
		}
	}

	Ok(())
}

pub mod production {
	use std::time::Duration;

	use crate::prelude::*;
	use camino::Utf8Path;
	use clap::Subcommand;
	use color_eyre::Report;
	use openssh::Session;
	use ysurreal::{args::ProductionDBConnection, server::SSHServerConnection};

	pub async fn handle(
		ssh_server: SSHServerConnection,
		production_command: ProductionCommand,
	) -> Result<(), Report> {
		match production_command {
			ProductionCommand::Kill => {
				let session = ssh_server.connect().await?;
				info!(
					message = "Killing all currently running surrealdb processes on the server",
					note = "The list returned will contain the PIDs of the killed processes"
				);
				sshserver(
					&session,
					"/root/.cargo/bin/nu",
					[
						"-c",
						r##"ps | find surreal | get pid | each {|pid| kill $pid; $pid }"##,
					],
				)
				.await?;
				info!("Killed surrealdb processes");

				Ok(())
			}
			ProductionCommand::Clean { surreal_path } => {
				let session = ssh_server.connect().await?;
				info!(
					message = "Cleaning the database folder on the server",
					?surreal_path
				);
				sshserver(&session, "rm", ["-rf", surreal_path.as_str()]).await?;
				info!("Cleaned database folder");

				Ok(())
			}
			ProductionCommand::Start {
				env_local_path,
				env_server_path,
				surreal_binary_path,
				nu_binary_path,
				surreal_data_path,
			} => {
				let session = ssh_server.connect().await?;
				info!(
					message = "Syncing the `env.nu` file",
					%env_local_path,
					%env_server_path
				);
				let mut cmd = std::process::Command::new("/usr/bin/scp");
				cmd.args([
					env_local_path.as_str(),
					&format!("{}:{}", ssh_server.ssh_name, env_server_path),
				]);
				let exit_status = cmd.spawn()?.wait()?;
				match exit_status.success() {
					true => info!("Securely copied `env.nu` file to server"),
					false => return Err(LocalError).wrap_err("Couldn't copy `env.nu` file to server"),
				}

				let sleep_duration = Duration::from_secs(2);
				info!(
					message = "Starting the database server",
					note = "Sleeping to allow time for DB to start before disconnecting",
					?sleep_duration,
					note = "You should see logs from surreal db in the console"
				);
				// sshserver(
				// 	&session,
				// 	nu_binary_path.as_str(),
				// 	["-c", &format!("source {env_server_path}; print \"Sourced the env file\"; {surreal_binary_path} start file://{surreal_data_path}")]
				// )
				// .await?;

				// uses provided env vars from env.nu, e.g. SURREAL_PASS
				let mut start_cmd = session.command(nu_binary_path.get().as_str());
				start_cmd.args([
						"-c",
						&format!(
							"source {env_server_path}; print \"Sourced the env file\"; {surreal_binary_path} start file://{surreal_data_path}",
						),
					]);
				let start_cmd: openssh::Child<&Session> = start_cmd.spawn().await?;
				std::thread::sleep(sleep_duration);
				// This will keep the remote process running according to the docs of [Child]
				// https://docs.rs/openssh/latest/openssh/struct.Child.html
				start_cmd.disconnect().await?;
				// start_cmd.wait().await?;

				info!(
					message = "Started database server",
					note =
						"The local handle has been disconnected so no *more* logs will appear in the console",
					note = "Logs should have appeared however, if they haven't something has gone wrong"
				);

				check(&session, &nu_binary_path.get()).await?;

				Ok(())
			}
			ProductionCommand::Check { nu_binary_path } => {
				let session = ssh_server.connect().await?;
				check(&session, &nu_binary_path.get()).await?;

				Ok(())
			}
			ProductionCommand::Import {
				db_connection,
				init_file,
			} => {
				// wish this would work
				// // has to use http so connect,
				// // see https://docs.rs/surrealdb/latest/surrealdb/struct.Surreal.html#support-1
				// let db = db_connection.connect_http().await?;
				// info!(message = "Importing file into production DB", ?init_file);

				// db.import(&init_file).await?;

				// info!("Finished import");

				let endpoint = format!("http://{}", db_connection.address);
				let endpoint = endpoint.as_str();
				let username = db_connection.username.as_str();
				let password = db_connection.password.as_str();
				let database = db_connection.database.as_str();
				let namespace = db_connection.namespace.as_str();
				info!(
					message = "Importing into production DB",
					?init_file,
					?endpoint
				);

				let surreal_bin_path = which("surreal").wrap_err("Couldn't find surreal binary path")?;
				let mut cmd = bossy::Command::pure(surreal_bin_path).with_args([
					"import",
					"--endpoint",
					endpoint,
					"--username",
					username,
					"--password",
					password,
					"--auth-level",
					"root",
					"--namespace",
					namespace,
					"--database",
					database,
					init_file.as_str(),
				]);
				cmd
					.run_and_wait()
					.wrap_err("Failed to run `surreal import`")?;
				info!("Finished interactive session");

				Ok(())
			}
			ProductionCommand::Connect { db_connection } => {
				info!(message = "Connected to production DB");
				let endpoint = format!("ws://{}", db_connection.address);
				let endpoint = endpoint.as_str();
				let username = db_connection.username.as_str();
				let password = db_connection.password.as_str();
				let database = db_connection.database.as_str();
				let namespace = db_connection.namespace.as_str();

				let surreal_bin_path = which("surreal").wrap_err("Couldn't find surreal binary path")?;
				let mut cmd = bossy::Command::pure(surreal_bin_path).with_args([
					"sql",
					"--endpoint",
					endpoint,
					"--username",
					username,
					"--password",
					password,
					"--auth-level",
					"root",
					"--namespace",
					namespace,
					"--database",
					database,
					"--pretty",
				]);
				cmd.run_and_wait().wrap_err("Failed to run `surreal sql`")?;
				info!("Finished interactive session");

				Ok(())
			}
		}
	}

	#[derive(Subcommand, Debug)]
	pub enum ProductionCommand {
		/// Stops running surrealdb instances
		Kill,

		/// [ProductionCommand::Kill]s, and deletes database data
		Clean {
			#[arg(long, env = "_SURREAL_DATA_PATH")]
			surreal_path: camino::Utf8PathBuf,
		},

		/// Starts running the database.
		///
		/// Should have already [ProductionCommand::Kill]ed, and maybe [ProductionCommand::Clean]ed
		Start {
			/// Path to the `env.nu` file to source before running `surreal start file:surreal.db`
			#[arg(long, env = "_ENV_LOCAL_PATH")]
			env_local_path: camino::Utf8PathBuf,

			/// Path to the `env.nu` file to copy and source on the server
			#[arg(long, env = "_ENV_SERVER_PATH")]
			env_server_path: camino::Utf8PathBuf,

			/// Path on server to where the `surreal` binary is located
			#[arg(long, env = "_ENV_SERVER_SURREAL_BINARY_PATH")]
			surreal_binary_path: camino::Utf8PathBuf,

			#[clap(flatten)]
			nu_binary_path: ServerNuBinaryPath,

			/// Path to folder holding actual surreal data
			#[arg(long, env = "_SURREAL_SERVER_DATA_PATH")]
			surreal_data_path: camino::Utf8PathBuf,
		},

		Check {
			#[clap(flatten)]
			nu_binary_path: ServerNuBinaryPath,
		},

		/// Imports the `db.surql` file into the database.
		///
		/// Assumes already [ProductionCommand::Start]ed.
		Import {
			#[clap(flatten)]
			db_connection: ProductionDBConnection,

			#[arg(long, env = "_SURREAL_INIT_LOCAL_PATH")]
			init_file: camino::Utf8PathBuf,
		},

		/// Runs an interactive shell for entering queries using `surreal sql --endpoint`
		Connect {
			#[clap(flatten)]
			db_connection: ProductionDBConnection,
		},
	}

	/// [clap] options
	#[derive(clap::Args, Debug, Clone)]
	pub struct ServerNuBinaryPath {
		/// Path on server to where the `nu` binary is located
		#[arg(long, env = "_ENV_SERVER_NU_BINARY_PATH")]
		nu_binary_path: camino::Utf8PathBuf,
	}

	impl ServerNuBinaryPath {
		pub fn get(&self) -> Utf8PathBuf {
			self.nu_binary_path.clone()
		}
	}

	#[derive(Debug, thiserror::Error)]
	#[error("Remote server process didn't exit with a `.success()` exit status :(")]
	struct SSHError;

	#[derive(Debug, thiserror::Error)]
	#[error("Local process didn't exit with a `.success()` exit status :(")]
	struct LocalError;

	async fn sshserver(
		session: &Session,
		cmd: &str,
		args: impl IntoIterator<Item = &str>,
	) -> Result<(), Report> {
		let mut cmd = session.command(cmd);
		cmd.args(args);
		let exit_status = cmd.status().await?;
		println!();
		info!(
			message = "Executed command, with stdout and stderr printing to console (i.e. inherited)",
			?exit_status
		);
		if !exit_status.success() {
			Err(SSHError).wrap_err("Couldn't execute command on server")
		} else {
			Ok(())
		}
	}

	/// Checks server is running
	pub async fn check(session: &Session, nu_binary_path: &Utf8Path) -> Result<(), Report> {
		info!("Running debug check");
		sshserver(
			session,
			nu_binary_path.as_str(),
			["-c", "lsof -i -P -n | find surreal"],
		)
		.await
		.wrap_err("SSH debug check failed to execute")
	}
}

pub mod testing {
	use std::time::Duration;

	use crate::prelude::*;
	use camino::Utf8PathBuf;
	use color_eyre::eyre::{Context, Report};
	use which::which;
	use ysurreal::args::TestingDBConnection;

	pub fn handle(testing_command: TestingCommand) -> Result<(), Report> {
		match testing_command {
			TestingCommand::Kill => {
				info!("Stopping all local surreal db instances");
				let exit_status = bossy::Command::pure(nu_bin_path()?.as_str()).with_args(
					["-c", r##"ps | filter {|ps| $ps.name == "/opt/homebrew/bin/surreal" } | get pid | each {|pid| kill $pid; $pid }"##]
				).run_and_wait()?;
				info!(
					message = "Finished stopping all local surreal db instances",
					?exit_status
				);

				Ok(())
			}
			TestingCommand::Start {
				connection_options,
				bind,
			} => {
				let bind = bind.as_str();
				let username = connection_options.username.as_str();
				let password = connection_options.password.as_str();
				let wait_duration = Duration::from_secs(2);
				info!(
					message = "Starting local surreal db instance",
					?bind,
					note = "Waiting for a bit after starting the server to see logs",
					?wait_duration
				);

				let surreal_bin_path = surreal_bin_path()?;
				let mut cmd = bossy::Command::pure(surreal_bin_path.as_str()).with_args([
					"start",
					"--bind",
					bind,
					"--strict",
					"--auth",
					"--username",
					username,
					"--password",
					password,
					// instead of file://foo.db
					"memory",
				]);
				let handle = cmd.run()?;
				std::thread::sleep(wait_duration);
				info!(
					message = "Finished starting local surreal db instance",
					note = "Detaching session for process, you should have seen logs above this message if everything went well",
					note = "bossy::Command doesn't like leaving stray processes, ignore the error just below this log"
				);
				drop(handle);

				info!("Running debug check");
				let nu_binary_path = nu_bin_path()?;
				bossy::Command::pure(nu_binary_path.as_str())
					.with_args(["-c", "lsof -i -P -n | find surreal"])
					.run_and_wait()?;

				Ok(())
			}
		}
	}

	#[derive(Subcommand, Debug)]
	pub enum TestingCommand {
		/// Starts dev server
		Start {
			/// Default 0.0.0.0:8000
			///
			/// Not localhost because this errors for some reason?
			#[arg(long, env = "_SURREAL_BIND_TESTING")]
			bind: String,

			#[clap(flatten)]
			connection_options: TestingDBConnection,
		},
		/// Stops dev server
		Kill,
	}

	/// Finds the path of the local surreal binary
	pub fn surreal_bin_path() -> Result<Utf8PathBuf, Report> {
		let path = which("surreal").wrap_err("Couldn't find surreal bin path")?;
		Utf8PathBuf::try_from(path).wrap_err("Couldn't convert path to Utf8PathBuf")
	}

	/// Finds the path of the local `nu` binary
	pub fn nu_bin_path() -> Result<Utf8PathBuf, Report> {
		let path = which("nu").wrap_err("Couldn't find nu bin path")?;
		Utf8PathBuf::try_from(path).wrap_err("Couldn't convert path to Utf8PathBuf")
	}

	pub fn check() -> Result<(), Box<dyn std::error::Error>> {
		let nu_binary_path = nu_bin_path()?;
		bossy::Command::pure(nu_binary_path.as_str())
			.with_args(["-c", "lsof -i -P -n | find surreal"])
			.run_and_wait()?;

		Ok(())
	}
}
