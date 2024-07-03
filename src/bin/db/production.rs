use std::time::Duration;

use crate::prelude::*;
use ymap::auth::config::ProductionConfig;
use ysurreal::config::{DBConnectRemoteConfig, DBRootCredentials, DBStartConfig};

#[derive(Subcommand, Debug, Clone)]
pub enum ProductionCommand {
	Kill,
	Clean,
	Start,
	Import,
	Connect,
	Check,
	Auth {
		#[clap(subcommand)]
		auth_subcommand: auth::AuthCommand,
	},
}

pub mod auth {
	use crate::prelude::*;
	use yauth::prelude::*;
	use ymap::auth::config::ProductionConfig;

	#[derive(Subcommand, Debug, Clone)]
	pub enum AuthCommand {
		SignUp {
			#[clap(flatten)]
			signup_options: yauth::signup::Signup,
		},
	}

	pub async fn handle(config: &ProductionConfig, command: &AuthCommand) -> Result<(), Report> {
		match command {
			AuthCommand::SignUp { signup_options } => {
				let db = config.connect_ws().await?;
				db.use_ns(config.primary_namespace())
					.use_db(config.primary_database())
					.await?;

				Ok(())
			}
		}
	}
}

use color_eyre::eyre::eyre;
use openssh::Session;
async fn sshserver(
	session: &Session,
	cmd: &str,
	args: impl IntoIterator<Item = &str>,
) -> Result<(), Report> {
	let mut cmd = session.command(cmd);
	cmd.args(args);
	let exit_status = cmd.status().await?;
	println!();
	info!(
		message = "Executed command, with stdout and stderr printing to console (i.e. inherited)",
		?exit_status
	);
	if !exit_status.success() {
		Err(eyre!(
			"Command through ssh connection failed: {:?}",
			exit_status
		))
	} else {
		Ok(())
	}
}

async fn kill(session: &Session, nu_binary_path: &Utf8Path) -> Result<(), Report> {
	sshserver(
		session,
		nu_binary_path.as_str(),
		[
			"-c",
			r##"ps | find surreal | get pid | each {|pid| kill $pid; $pid }"##,
		],
	)
	.await
}

async fn clean(session: &Session, data_path: &Utf8Path) -> Result<(), Report> {
	sshserver(session, "rm", ["-rf", data_path.as_str()]).await
}

async fn start(
	session: &Session,
	config: &ProductionConfig,
	wait_duration: Duration,
) -> Result<(), Report> {
	let surreal_bin_path = config.surreal_binary_path.as_str();
	let args = config.get_cli_args().join(" ");
	let server_cmd = format!("{surreal_bin_path} start {args}");
	let mut start_cmd = session.command(config.nu_binary_path.as_str());
	start_cmd.args(["-c", &server_cmd]);
	let start_cmd = start_cmd.spawn().await?;

	std::thread::sleep(wait_duration);

	start_cmd.disconnect().await?;

	Ok(())
}

async fn check(session: &Session, nu_binary_path: &Utf8Path) -> Result<(), Report> {
	sshserver(
		session,
		nu_binary_path.as_str(),
		["-c", "lsof -i -P -n | find surreal"],
	)
	.await?;

	Ok(())
}

pub async fn handle(config: &ProductionConfig, command: &ProductionCommand) -> Result<(), Report> {
	match command {
		ProductionCommand::Auth { auth_subcommand } => auth::handle(config, auth_subcommand).await,
		ProductionCommand::Kill => {
			let session = config.ssh().await?;
			info!("Killing all surreal processes on the server");
			kill(&session, &config.nu_binary_path).await?;

			Ok(())
		}
		ProductionCommand::Clean => {
			let session = config.ssh().await?;
			info!("Cleaning data path on the server");
			clean(&session, &config.surreal_data_path).await?;

			Ok(())
		}
		ProductionCommand::Start => {
			let session = config.ssh().await?;
			info!("Starting surrealdb instance on server");
			start(&session, config, Duration::from_secs(2)).await?;
			check(&session, &config.nu_binary_path).await?;

			Ok(())
		}
		ProductionCommand::Import => {
			let db = config.connect_ws().await?;
			config.root_sign_in(&db).await?;
			config.root_init(&db).await?;
			db.use_ns(config.primary_namespace())
				.use_db(config.primary_database())
				.await?;

			Ok(())
		}
		ProductionCommand::Connect => {
			bossy::Command::pure(which("surreal").wrap_err("Cannot find local surreal binary")?)
				.with_arg("sql")
				.with_args(config.get_sql_cli_args())
				.run_and_wait()
				.wrap_err("Failed to run surreal sql")?;

			Ok(())
		}
		ProductionCommand::Check => {
			let session = config.ssh().await?;
			check(&session, &config.nu_binary_path).await?;

			Ok(())
		}
	}
}
