use std::time::Duration;

use crate::args::TestingDBConnection;
use crate::prelude::*;
use camino::Utf8PathBuf;
use color_eyre::eyre::{Context, Report};
use which::which;

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
