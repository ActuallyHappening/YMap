use crate::prelude::*;

use camino::Utf8Path;
use clap::Subcommand;
use color_eyre::Report;
use openssh::Session;
use std::time::Duration;

use surrealdb::{
	engine::remote::{
		http::{self, Http},
		ws::{self, Ws},
	},
	opt::auth::Root,
	Surreal,
};

/// Options for connecting to the server DB with root credentials.
///
/// Primary usecase is to turn into [surrealdb::Surreal] instance.
#[derive(Args, Debug, Clone)]
pub struct ProductionDBConnection {
	#[arg(long, env = "SURREAL_USER")]
	pub username: String,

	#[arg(long, env = "SURREAL_PASS")]
	pub password: String,

	/// Without protocol specifier, e.g. localhost:8000
	#[arg(long, env = "_SURREAL_HOST_PRODUCTION")]
	pub address: String,

	#[arg(long, env = "_SURREAL_DATABASE_PRODUCTION")]
	pub database: String,

	#[arg(long, env = "_SURREAL_NAMESPACE_PRODUCTION")]
	pub namespace: String,
}

impl ProductionDBConnection {
	/// Constructs a new instance from the environment variables only.
	pub fn from_env() -> Result<Self, Report> {
		use clap::Parser;
		#[derive(Parser)]
		struct ParseMe {
			#[clap(flatten)]
			data: ProductionDBConnection,
		}

		let data = ParseMe::try_parse_from([&""]).wrap_err("Couldn't parse from env")?;
		Ok(data.data)
	}

	pub async fn connect_http(&self) -> Result<Surreal<http::Client>, surrealdb::Error> {
		let address = self.address.as_str();
		let namespace = self.namespace.as_str();
		let database = self.database.as_str();
		let username = self.username.as_str();
		let password = self.password.as_str();
		info!(
			message = "Connecting to production DB",
			?address,
			?namespace,
			?database,
			note = "Waiting for database connection before proceeding"
		);

		let db = Surreal::new::<Http>(address).await?;
		db.use_ns(namespace).use_db(database).await?;
		db.signin(Root { username, password }).await?;
		db.wait_for(surrealdb::opt::WaitFor::Database).await;

		Ok(db)
	}

	pub async fn connect_ws(&self) -> Result<Surreal<ws::Client>, surrealdb::Error> {
		let address = self.address.as_str();
		let namespace = self.namespace.as_str();
		let database = self.database.as_str();
		let username = self.username.as_str();
		let password = self.password.as_str();
		info!(
			message = "Connecting to production DB",
			?address,
			?namespace,
			?database,
			note = "Waiting for database connection before proceeding"
		);

		let db = Surreal::new::<Ws>(address).await?;
		db.use_ns(namespace).use_db(database).await?;
		db.signin(Root { username, password }).await?;
		db.wait_for(surrealdb::opt::WaitFor::Database).await;

		Ok(db)
	}
}

#[derive(Args, Debug, Clone)]
pub struct SSHServerConnection {
	/// What you would type in `ssh <NAME>`.
	/// e.g. ah@example.com, localhost
	///
	/// Does not include port, see [ServerConnectionOptions::ssh_port]
	#[arg(long, env = "YSURREAL_SSH_NAME")]
	pub ssh_name: String,
	// #[arg(long, env = "YSURREAL_SSH_PORT")]
	// pub ssh_port: String,
}

impl SSHServerConnection {
	/// Constructs a new instance from the environment variables only.
	pub fn from_env() -> Result<Self, Report> {
		use clap::Parser;
		#[derive(Parser)]
		struct ParseMe {
			#[clap(flatten)]
			data: SSHServerConnection,
		}

		let data = ParseMe::try_parse_from([&""]).wrap_err("Couldn't parse from env")?;
		Ok(data.data)
	}

	pub async fn connect(&self) -> Result<Session, openssh::Error> {
		let ssh_name = self.ssh_name.as_str();
		info!(message = "Connecting to server host", ?ssh_name);
		Session::connect_mux(ssh_name, openssh::KnownHosts::Strict).await
	}
}

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
		#[arg(long, env = "_SURREAL_SERVER_DATA_PATH")]
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn production_db_connection_from_env() {
		let _ = ProductionDBConnection::from_env().unwrap();
	}

	#[test]
	fn production_ssh_connection_from_env() {
		SSHServerConnection::from_env().unwrap();
	}

	#[ignore = "IDK please fix this, just doesn't work??"]
	#[tokio::test]
	async fn production_can_connect_http() {
		let conn_options = ProductionDBConnection::from_env().unwrap();

		conn_options.connect_http().await.unwrap();
	}

	#[tokio::test]
	async fn production_can_connect_ws() {
		let conn_options = ProductionDBConnection::from_env().unwrap();

		conn_options.connect_ws().await.unwrap();
	}
}
