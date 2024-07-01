use clap::{Parser, Subcommand};
use openssh::Session;
use surrealdb::{engine::remote::ws::Ws, Surreal};
use tracing::*;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
	#[clap(flatten)]
	pub connection: ysurreal::args::SurrealConnectionOptions,

	#[clap(flatten)]
	pub server: ysurreal::server::ServerConnectionOptions,

	#[clap(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
	Reset,
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

async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
	match cli.command {
		Commands::Reset => {
			let session = cli.server.connect().await?;

			
		}
	}

	Ok(())
}