use std::time::Duration;

use crate::prelude::*;
use ysurreal::config::{RootConnectDBConfig, StartDBConfig};

#[derive(Args, Debug, Clone)]
pub struct ProductionConfig {
	#[arg(long, default_value_t = { Secrets::ssh_name() })]
	ssh_name: String,

	#[arg(long, default_value_t = Utf8PathBuf::from("/root/home/YMap/surreal.db"))]
	surreal_data_path: Utf8PathBuf,

	#[arg(long, default_value_t = Utf8PathBuf::from("/usr/local/bin/surreal"))]
	surreal_binary_path: Utf8PathBuf,
}

impl StartDBConfig for ProductionConfig {
	fn root_password(&self) -> String {
		Secrets::production_password()
	}

	fn primary_namespace(&self) -> String {
		"production".into()
	}

	fn primary_database(&self) -> String {
		"production".into()
	}

	fn bind_port(&self) -> u16 {
		42069
	}

	fn db_type(&self) -> ysurreal::config::StartDBType {
		ysurreal::config::StartDBType::File {
			data_path: Utf8PathBuf::from("/root/home/YMap/surreal.db"),
		}
	}
}

impl RootConnectDBConfig for ProductionConfig {
	fn connect_host(&self) -> String {
		"actually-happening.foundation".into()
	}

	fn connect_port(&self) -> u16 {
		42069
	}
}

impl ProductionConfig {
	pub async fn ssh(&self) -> Result<Session, openssh::Error> {
		let ssh_name = self.ssh_name.as_str();
		info!(message = "Connecting to server host", ?ssh_name);
		Session::connect_mux(ssh_name, openssh::KnownHosts::Strict).await
	}
}

#[derive(Subcommand, Debug, Clone)]
pub enum ProductionCommand {
	Kill,
	Clean,
	Start,
	Import,
	Connect,
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

async fn kill(session: &Session) -> Result<(), Report> {
	sshserver(
		session,
		"/root/.cargo/bin/nu",
		[
			"-c",
			r##"ps | find surreal | get pid | each {|pid| kill $pid; $pid }"##,
		],
	)
	.await
}

async fn clean(session: &Session, data_path: &Utf8Path) -> Result<(), Report> {
	sshserver(session, "rm", ["rf", data_path.as_str()]).await
}

async fn start(
	session: &Session,
	config: &ProductionConfig,
	wait_duration: Duration,
) -> Result<(), Report> {
	let surreal_bin_path = config.surreal_binary_path.as_str();
	let args = config.get_cli_args().join(" ");
	let server_cmd = format!("{surreal_bin_path} start {args}");
	let mut start_cmd = session.command("nu");
	start_cmd.args(["-c", &server_cmd]);
	let start_cmd = start_cmd.spawn().await?;

	std::thread::sleep(wait_duration);

	start_cmd.disconnect().await?;

	Ok(())
}

async fn check(session: &Session) -> Result<(), Report> {
	sshserver(session, "nu", ["-c", "lsof -i -P -n | find surreal"]).await?;

	Ok(())
}

pub async fn handle(config: &ProductionConfig, command: &ProductionCommand) -> Result<(), Report> {
	match command {
		ProductionCommand::Kill => {
			let session = config.ssh().await?;
			info!("Killing all surreal processes on the server");
			kill(&session).await?;

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
			check(&session).await?;

			Ok(())
		}
		ProductionCommand::Import => {
			let db = config.connect_ws().await?;

			Ok(())
		}
	}
}