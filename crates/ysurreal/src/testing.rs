use std::time::Duration;

use crate::prelude::*;
use camino::Utf8PathBuf;
use clap::Parser;
use color_eyre::eyre::{Context, Report};
use surrealdb::{
	engine::remote::{
		http::{self, Http},
		ws::{self, Ws},
	},
	Surreal,
};
use which::which;

/// Options for connecting to the server DB.
///
/// Does *not automatically sign into anything*. See [yauth] for custom signin.
/// Primary usecase is to turn into [surrealdb::Surreal] instance.
///
/// Also has root credentials, but DOESN'T automatically sign into as root.
///
/// See also [ProductionDBConnection] for root signin.
#[derive(Args, Debug, Clone)]
pub struct TestingDBConnection {
	#[arg(long, env = "_SURREAL_USER_TESTING")]
	pub username: String,

	#[arg(long, env = "_SURREAL_PASS_TESTING")]
	pub password: String,

	#[arg(long, env = "_SURREAL_PORT_TESTING")]
	pub port: String,

	/// Without protocol specifier, e.g. localhost:8000
	#[arg(long, env = "_SURREAL_HOST_TESTING")]
	pub address: String,

	#[arg(long, env = "_SURREAL_DATABASE_TESTING")]
	pub database: String,

	#[arg(long, env = "_SURREAL_NAMESPACE_TESTING")]
	pub namespace: String,
}

impl TestingDBConnection {
	/// Constructs a new instance from the environment variables only.
	pub fn from_env() -> Result<Self, Report> {
		#[derive(Parser)]
		struct ParseMe {
			#[clap(flatten)]
			data: TestingDBConnection,
		}

		let data = ParseMe::try_parse_from([&""]).wrap_err("Couldn't parse from env")?;
		Ok(data.data)
	}

	pub async fn connect_http(&self) -> Result<Surreal<http::Client>, surrealdb::Error> {
		let address = self.address.as_str();
		let namespace = self.namespace.as_str();
		let database = self.database.as_str();
		// let username = self.username.as_str();
		// let password = self.password.as_str();
		info!(
			message = "Connecting to testing DB",
			?address,
			?namespace,
			?database,
			note = "Waiting for database connection before proceeding"
		);

		let db = Surreal::new::<Http>(address).await?;
		db.use_ns(namespace).use_db(database).await?;
		db.wait_for(surrealdb::opt::WaitFor::Database).await;

		Ok(db)
	}

	pub async fn connect_ws(&self) -> Result<Surreal<ws::Client>, surrealdb::Error> {
		let address = self.address.as_str();
		let namespace = self.namespace.as_str();
		let database = self.database.as_str();
		// let username = self.username.as_str();
		// let password = self.password.as_str();
		info!(
			message = "Connecting to testing DB",
			?address,
			?namespace,
			?database,
			note = "Waiting for database connection before proceeding"
		);

		let db = Surreal::new::<Ws>(address).await?;
		db.use_ns(namespace).use_db(database).await?;
		db.wait_for(surrealdb::opt::WaitFor::Database).await;

		Ok(db)
	}
}

pub fn handle(testing_command: TestingCommand) -> Result<(), Report> {
	match testing_command {
		TestingCommand::Kill => {
			info!("Stopping all local surreal db instances");
			let exit_status = bossy::Command::pure(nu_bin_path()?.as_str())
				.with_args([
					"-c",
					r##"ps | filter {|ps| $ps.name == "surreal" } | get pid | each {|pid| kill $pid; $pid }"##,
				])
				.run_and_wait()?;
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

			check()?;

			Ok(())
		}
		TestingCommand::Check => {
			check()?;

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

	/// Runs debug check
	Check,

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

pub fn check() -> Result<(), Report> {
	let nu_binary_path = nu_bin_path()?;
	bossy::Command::impure(nu_binary_path.as_str())
		.with_args(["-c", "^lsof -i -P -n | find surreal"])
		.run_and_wait()
		.wrap_err("Didn't execute `lsof` successfully")?;

	Ok(())
}

#[cfg(test)]
mod test {
	use super::*;

	/// Requires env vars to be sourced from env.nu first
	#[test]
	fn testing_db_connection_from_env() {
		match TestingDBConnection::from_env() {
			Ok(_) => {}
			Err(err) => {
				eprintln!(
					"_SURREAL_USER_TESTING: {:?}",
					std::env::var("_SURREAL_USER_TESTING")
				);
				panic!("{}", err)
			}
		}
	}
}
