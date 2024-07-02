use std::str::FromStr as _;

use clap::{Parser, Subcommand};
use tracing::*;
use yauth::{
	prelude::*,
	types::{Email, Password, Username},
};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
	#[clap(flatten)]
	connection_options: ysurreal::testing::TestingDBConnection,

	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
	Signup {
		/// Email to sign up with.
		/// 
		/// Must be unique
		#[arg(long, default_value_t = { Email::from_str("ah@example.com").unwrap() })]
		email: Email,

		/// Arbitrary username
		#[arg(long)]
		username: Username,

		/// Plaintext password
		#[arg(long, default_value_t = { Password::from_str("long enough").unwrap() } )]
		password: Password,

		#[arg(long, default_value_t = String::from("user"))]
		scope: String,

		#[arg(long, default_value_t = String::from("user"))]
		users_table: String,

		#[arg(long, env = "_SURREAL_DATABASE_TESTING")]
		database: String,

		#[arg(long, env = "_SURREAL_NAMESPACE_TESTING")]
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

fn install_tracing() {
	use tracing_error::ErrorLayer;
	use tracing_subscriber::prelude::*;
	use tracing_subscriber::{fmt, EnvFilter};

	let fmt_layer = fmt::layer().with_target(false);
	let filter_layer = EnvFilter::try_from_default_env()
		.or_else(|_| EnvFilter::try_new("info,yauth=trace"))
		.unwrap();

	tracing_subscriber::registry()
		.with(filter_layer)
		.with(fmt_layer)
		.with(ErrorLayer::default())
		.init();
}

async fn run() -> Result<(), yauth::AuthError> {
	color_eyre::install().expect("Failed to install color_eyre");
	install_tracing();

	info!("Starting debug yauth CLI");

	let cli = Cli::parse();

	let db = cli.connection_options.connect_ws().await?;

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
				db: &db,
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
