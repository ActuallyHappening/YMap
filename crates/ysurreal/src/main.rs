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

pub mod production;

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
