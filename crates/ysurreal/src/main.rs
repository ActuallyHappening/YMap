use clap::{Parser, Subcommand};
use openssh::Session;
use tracing::*;
use tracing_subscriber::EnvFilter;
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
		#[arg(long, env = "_SURREAL_PATH")]
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
		#[arg(long, env = "_ENV_SURREAL_BINARY_PATH")]
		surreal_binary_path: camino::Utf8PathBuf,

		/// Path on server to where the `nu` binary is located
		#[arg(long, env = "_ENV_NU_BINARY_PATH")]
		nu_binary_path: camino::Utf8PathBuf,

		/// Path to folder holding actual surreal data
		#[arg(long, env = "_SURREAL_DATA_PATH")]
		surreal_data_path: camino::Utf8PathBuf,
	},

	/// Imports the `db.surql` file into the database.
	///
	/// Assumes already [ProductionCommand::Start]ed.
	Import {
		#[clap(flatten)]
		db_connection: ProductionDBConnection,

		#[arg(long, env = "_SURREAL_INIT_PATH_LOCAL")]
		init_file: camino::Utf8PathBuf,
	},
}

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt()
		.with_env_filter(
			EnvFilter::builder()
				.try_from_env()
				.or_else(|_| EnvFilter::try_new("info,ysurreal=trace"))
				.unwrap(),
		)
		.init();

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
			let session = ssh_server.connect().await?;
			match production_command {
				ProductionCommand::Kill => {
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
					info!(
						message = "Syncing the `env.nu` file",
						?env_local_path,
						?env_server_path
					);
					let mut cmd = std::process::Command::new("scp");
					cmd.args([
						env_local_path.as_str(),
						&format!("{}:{}", ssh_server.ssh_name, env_server_path),
					]);
					let exit_status = cmd.spawn()?.wait()?;
					match exit_status.success() {
						true => info!("Securely copied `env.nu` file to server"),
						false => return Err(Box::new(SSHError)),
					}

					info!("Starting the database server");
					sshserver(
						&session,
						nu_binary_path.as_str(),
						["-c", &format!("source {env_server_path}; print 'Sourced the env file'; {surreal_binary_path} start file://{surreal_data_path}")]
					)
					.await?;
					info!("Started database server");
				}
				_ => todo!(),
			}
		}
	}

	Ok(())
}
