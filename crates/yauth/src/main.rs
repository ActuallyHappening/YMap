use std::str::FromStr as _;

use clap::{Parser, Subcommand};
use surrealdb::{engine::remote::ws::Ws, Surreal};
use tracing::*;
use tracing_subscriber::EnvFilter;
use yauth::{
	prelude::*,
	types::{Email, Password, Username},
};

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
		username: Username,

		#[arg(long, default_value_t = { Email::from_str("ah@example.com").unwrap() })]
		email: Email,

		#[arg(long, default_value_t = { Password::from_str("123password").unwrap() } )]
		password: Password,

		#[arg(long, default_value_t = String::from("user"))]
		scope: String,

		#[arg(long, default_value_t = String::from("user"))]
		users_table: String,

		#[arg(long, default_value_t = String::from("production"))]
		database: String,

		#[arg(long, default_value_t = String::from("production"))]
		namespace: String,
	},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing_subscriber::fmt()
		.with_env_filter(
			EnvFilter::builder()
				.try_from_env()
				.or_else(|_| EnvFilter::try_new("info,yauth=trace"))
				.unwrap(),
		)
		.init();

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
			scope,
		} => {
			let auth_con = AuthConnection {
				db: &db_con,
				namespace,
				database,
				users_table,
				scope,
			};
			auth_con
				.signup(yauth::signup::Signup {
					username,
					password,
					email,
				})
				.await?;
		}
	}

	Ok(())
}
