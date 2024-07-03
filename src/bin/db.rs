pub mod prelude {
	pub(crate) use clap::{Parser, Subcommand};
	pub(crate) use color_eyre::eyre::Report;
	pub(crate) use tracing::*;
}

#[path = "db/production.rs"]
pub mod production;

use ymap::secrets::{Secrets, SecretsTemplate};

use crate::prelude::*;

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
		#[arg(long, default_value_t = Secrets::ssh_name())]
		ssh_server: String,

		#[clap(subcommand)]
		production_command: production::ProductionCommand,
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

async fn run(cli: Cli) -> Result<(), Report> {
	match cli.command {
		Commands::Production {
			ssh_server,
			production_command,
		} => {
			production::handle(&ssh_server, &production_command).await?;
		}
	}

	Ok(())
}
