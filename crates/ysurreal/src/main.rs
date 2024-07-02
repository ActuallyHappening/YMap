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
	/// Assumes already [ProductionCommand::Reset], or at least [ProductionCommand::Kill]ed.
	Start,

	/// Imports the `db.surql` file into the database.
	///
	/// Assumes already [ProductionCommand::Start]ed.
	Import {
		#[clap(flatten)]
		db_connection: ProductionDBConnection,
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
#[error("Process didn't exit with a `.success()` exit status :(")]
struct SSHError;

async fn sshserver(
	session: &Session,
	cmd: &str,
	args: impl IntoIterator<Item = &str>,
) -> Result<(), Box<dyn std::error::Error>> {
	let mut cmd = session.command(cmd);
	cmd.args(args);
	let exit_status = cmd.status().await?;
	print!("\n");
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
					info!("Killing all currently running surrealdb processes on the server");
					sshserver(
						&session,
						"/root/.cargo/bin/nu",
						[
							"-c",
							r##"ps | filter {|ps| $ps.name == "/usr/local/bin/surreal"} | get pid | each {|pid| kill $pid; $pid }"##,
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
				_ => todo!(),
			}
		}
	}

	Ok(())
}
