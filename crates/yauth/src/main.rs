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
	#[clap(flatten)]
	connection_options: ysurreal::args::SurrealConnectionOptions,

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

		#[arg(long, default_value_t = { Password::from_str("long enough").unwrap() } )]
		password: Password,

		#[arg(long, default_value_t = String::from("user"))]
		scope: String,

		#[arg(long, default_value_t = String::from("user"))]
		users_table: String,

		#[arg(long, env = "SURREAL_DATABASE")]
		database: String,

		#[arg(long, env = "SURREAL_NAMESPACE")]
		namespace: String,
	},
}

#[tokio::main]
async fn main() {
	let main = run().await;
	match main {
		Ok(_) => info!("yauth CLI completed successfully"),
		Err(err) => {
			eprintln!("{}", err);
		}
	}
}

async fn run() -> Result<(), yauth::AuthError> {
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

	let db_con = Surreal::new::<Ws>(cli.connection_options.connection).await?;

	db_con.use_ns(cli.connection_options.namespace).use_db(cli.connection_options.database).await?;

	info!("Waiting for DB to connect ...");
	db_con.wait_for(surrealdb::opt::WaitFor::Database).await;

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
