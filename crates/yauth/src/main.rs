use clap::{Parser, Subcommand};
use surrealdb::{engine::remote::ws::Ws, Surreal};
use tracing::*;
use yauth::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
	#[arg(long, env = "_SURREAL_CONNECTION")]
	connection: String,

	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
	Signup {
		#[arg(long)]
		username: String,

		#[arg(long, default_value_t = String::from("ah@example.com"))]
		email: String,

		#[arg(long, default_value_t = String::from("123password"))]
		password: String,

		#[arg(long, default_value_t = String::from("user"))]
		users_table: String,

		#[arg(long, default_value_t = String::from("production"))]
		namespace: String,

		#[arg(long, default_value_t = String::from("production"))]
		database: String,
	},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing_subscriber::fmt::init();

	info!("Starting debug yauth CLI");

	let cli = Cli::parse();

	let db_con = Surreal::new::<Ws>(cli.connection).await?;

	match cli.command {
		Commands::Signup {
			username,
			email,
			password,
			users_table,
			namespace,
			database,
		} => {
			
		}
	}

	Ok(())
}
