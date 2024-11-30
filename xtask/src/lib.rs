use color_eyre::eyre::eyre;
use openssh::{KnownHosts, Session, Stdio};

pub mod prelude {
	pub use crate::{Error, Result};
	pub use color_eyre::eyre::Context as _;
	pub use color_eyre::{Section as _, SectionExt as _};
	pub use tracing::*;
}
use prelude::*;
pub mod cli;

pub type Error = color_eyre::Report;
pub type Result<T> = color_eyre::Result<T>;

/// This is the name of the ssh server to connect to, which you will
/// need to setup on your machine
const SSH_SERVER_NAME: &str = "ymap";

#[derive(Debug)]
pub struct ServerSession {
	session: openssh::Session,
}

impl ServerSession {
	pub async fn connect_to_server() -> Result<ServerSession> {
		Ok(ServerSession {
			session: Session::connect(SSH_SERVER_NAME, KnownHosts::Strict).await?,
		})
	}

	async fn cmd(&self, cmd: &str) -> Result<String> {
		let cmd = self.session.command(cmd).output().await?;
		let stdout = String::from_utf8(cmd.stdout)
			.wrap_err("Command executed on server didn't return valid UTF8 in its standard out")?;
		let stderr = String::from_utf8(cmd.stderr)
			.wrap_err("Command executed on server didn't return valid UTF8 in its standard error")?;
		if !cmd.status.success() {
			return Err(
				eyre!("Executing command on server failed")
					.with_section(move || stdout.trim().to_string().header("Stdout:"))
					.with_section(move || stderr.trim().to_string().header("Stderr:")),
			);
		}

		Ok(stdout.to_string())
	}

	async fn cmd_num(&self, cmd: &str, task: &'static str) -> Result<bool> {
		match self.cmd(cmd).await?.parse::<u8>() {
			Ok(num) => Ok(Self::status_from_num(num, task)),
			Err(e) => Err(e).wrap_err(format!(
				"Expected a number from command output but failed to parse as integer"
			)),
		}
	}

	async fn background_cmd(&self, cmd: &str) -> Result<()> {
		self
			.session
			.command(cmd)
			.stdin(Stdio::null())
			.stdout(Stdio::null())
			.stderr(Stdio::null())
			.spawn()
			.await?;
		Ok(())
	}

	fn status_from_num(num: u8, task: &'static str) -> bool {
		match num {
			num if num == 0 => false,
			num if num == 1 => true,
			num => {
				warn!(
					message = "Too many surreal instances on server",
					?task,
					?num
				);
				true
			}
		}
	}

	pub async fn scan(&self) -> Result<bool> {
		self.cmd_num(db_credentials::SEARCH_COMMAND, "scan").await
	}

	pub async fn start(&self) -> Result<()> {
		self
			.background_cmd(db_credentials::START_COMMAND)
			.await
			.wrap_err("Couldn't start surreal server")?;
		Ok(())
	}

	pub async fn clean(&self) -> Result<()> {
		self
			.cmd(db_credentials::CLEAN_COMMAND)
			.await
			.wrap_err("Couldn't clean surreal server")?;
		Ok(())
	}

	pub async fn kill(&self) -> Result<bool> {
		self.cmd_num(db_credentials::KILL_COMMAND, "kill").await
	}
}
