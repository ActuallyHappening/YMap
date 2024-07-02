use std::time::Duration;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Context;
use openssh::Session;
use tracing::*;
use which::which;
use ysurreal::args::ProductionDBConnection;

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
		production_command: ProductionCommand,
	},
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

		/// Path on server to where the `nu` binary is located
		#[arg(long, env = "_ENV_SERVER_NU_BINARY_PATH")]
		nu_binary_path: camino::Utf8PathBuf,

		/// Path to folder holding actual surreal data
		#[arg(long, env = "_SURREAL_SERVER_DATA_PATH")]
		surreal_data_path: camino::Utf8PathBuf,
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
) -> Result<(), Box<dyn std::error::Error>> {
	let mut cmd = session.command(cmd);
	cmd.args(args);
	let exit_status = cmd.status().await?;
	println!();
	info!(
		message = "Executed command, with stdout and stderr printing to console (i.e. inherited)",
		?exit_status
	);
	if !exit_status.success() {
		Err(Box::new(SSHError))
	} else {
		Ok(())
	}
}

async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
	match cli.command {
		Commands::Production {
			ssh_server,
			production_command,
		} => {
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
				}
				ProductionCommand::Clean { surreal_path } => {
					let session = ssh_server.connect().await?;
					info!(
						message = "Cleaning the database folder on the server",
						?surreal_path
					);
					sshserver(&session, "rm", ["-rf", surreal_path.as_str()]).await?;
					info!("Cleaned database folder");
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
						false => return Err(Box::new(LocalError)),
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
					let mut start_cmd = session.command(nu_binary_path.as_str());
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

					info!("Running debug check");
					sshserver(
						&session,
						nu_binary_path.as_str(),
						["-c", "lsof -i -P -n | find surreal"],
					)
					.await?;
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
					info!(message = "Importing into production DB", ?init_file, ?endpoint);

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
					cmd.run_and_wait().wrap_err("Failed to run `surreal import`")?;
					info!("Finished interactive session");
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
				}
			}
		}
	}

	Ok(())
}
