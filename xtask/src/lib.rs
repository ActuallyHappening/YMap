use color_eyre::eyre::eyre;
use openssh::{KnownHosts, Session};

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

#[derive(Debug)]
pub struct ServerSession {
    session: openssh::Session,
}

impl ServerSession {
    pub async fn connect_to_server() -> Result<ServerSession> {
        Ok(ServerSession {
            session: Session::connect("ymap", KnownHosts::Strict).await?,
        })
    }

    async fn cmd(&self, cmd: &str) -> Result<String> {
        let cmd = self.session.command(cmd).output().await?;
        let stdout = String::from_utf8(cmd.stdout)
            .wrap_err("Command executed on server didn't return valid UTF8 in its standard out")?;
        let stderr = String::from_utf8(cmd.stderr).wrap_err(
            "Command executed on server didn't return valid UTF8 in its standard error",
        )?;
        if !cmd.status.success() {
            return Err(eyre!("Executing command on server failed")
                .with_section(move || stdout.trim().to_string().header("Stdout:"))
                .with_section(move || stderr.trim().to_string().header("Stderr:")));
        }

        Ok(stdout.to_string())
    }

    pub async fn is_surreal_running(&self) -> Result<bool> {
        match self.cmd("ps | find surreal | length").await?.parse::<u8>() {
            Ok(num) if num == 0 => Ok(false),
            Ok(num) if num == 1 => Ok(true),
            Ok(num) => {
                warn!(message = "Too many surreal isntances in server", ?num);
                Ok(true)
            }
            Err(e) => Err(e).wrap_err("Return of script wasn't a number"),
        }
    }
}
