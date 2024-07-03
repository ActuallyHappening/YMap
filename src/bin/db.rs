pub mod prelude {
	pub(crate) use clap::{Parser, Subcommand, Args};
	pub(crate) use color_eyre::eyre::Report;
	pub(crate) use tracing::*;
	pub(crate) use ymap::secrets::{Secrets, SecretsTemplate};
	pub(crate) use camino::{Utf8Path, Utf8PathBuf};
	pub(crate) use color_eyre::eyre::WrapErr;
	pub(crate) use which::which;
}

#[path = "db/production.rs"]
pub mod production;

use crate::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about = "DB controller script")]
pub struct Cli {
	#[clap(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
	/// Executes commands on the server, see [ProductionCommand]
	Production {
		#[clap(flatten)]
		production_config: production::ProductionConfig,

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
		.or_else(|_| EnvFilter::try_new("info,ymap=trace,ysurreal=trace"))
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
			eprintln!("Error running DB script: {}", err);
		}
	}
}

async fn run(cli: Cli) -> Result<(), Report> {
	match cli.command {
		Commands::Production {
			production_config,
			production_command,
		} => {
			production::handle(&production_config, &production_command).await?;
		}
	}

	Ok(())
}
