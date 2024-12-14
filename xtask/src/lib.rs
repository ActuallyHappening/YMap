use std::process::{ExitStatus, Output};

use cli::{CommandSpawnOptions, ServerCommands};
use color_eyre::eyre::eyre;
use openssh::{KnownHosts, OwningCommand, Session, Stdio};

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

    fn parse_into_command(&self, cmd: Vec<&str>) -> Result<OwningCommand<&openssh::Session>> {
        let (cmd, args) = (
            *cmd.first()
                .ok_or(eyre!("Must provide a command to execute"))?,
            &cmd[1..],
        );
        let mut cmd_builder = self.session.command(cmd);
        cmd_builder.args(args);
        Ok(cmd_builder)
    }

    fn handle_status_errors(status: ExitStatus) -> Result<()> {
        if !status.success() {
            return Err(eyre!("Command executed on server exited badly"));
        }
        Ok(())
    }

    /// Raises an error if it exited badly,
    /// else returns the stdout
    fn handle_output_errors(cmd: Output) -> Result<String> {
        let stdout = String::from_utf8(cmd.stdout)
            .wrap_err("Command executed on server didn't return valid UTF8 in its standard out")?;
        let stderr = String::from_utf8(cmd.stderr).wrap_err(
            "Command executed on server didn't return valid UTF8 in its standard error",
        )?;
        debug!(?stdout, ?stderr);
        if !cmd.status.success() {
            return Err(eyre!("Executing command on server failed")
                .with_section(move || stdout.trim().to_string().header("Stdout:"))
                .with_section(move || stderr.trim().to_string().header("Stderr:")));
        }

        Ok(stdout.to_string())
    }

    async fn cmd(&self, cmd: Vec<&str>) -> Result<String> {
        info!(message = "Executing on server", ?cmd);
        let cmd = self.parse_into_command(cmd)?.output().await?;
        Self::handle_output_errors(cmd)
    }

    async fn cmd_num(&self, cmd: Vec<&str>, task: ServerCommands) -> Result<bool> {
        let output = self.cmd(cmd).await?;
        match output.trim().parse::<u8>() {
            Ok(num) => Ok(Self::status_from_num(num, task)),
            Err(e) => Err(e)
                .wrap_err(format!(
                    "Expected a number from command {:?} output but failed to parse as integer",
                    task
                ))
                .with_note(|| output.header("Command output:")),
        }
    }

    async fn cmd_persistent(&self, cmd: Vec<&str>, options: CommandSpawnOptions) -> Result<()> {
        match options.in_background() {
            true => {
                info!(
                    message = "Executing on server in the background",
                    ?cmd,
                    note = "Errors will not be reported by default"
                );
                self.parse_into_command(cmd)?
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .await?;
                Ok(())
            }
            false => {
                info!(
                    message = "Executing on the server piping output to current terminal session",
                    note = "The server process will continue to run on the server even when this connection is closed",
                    note = "You will have to manually kill this process however",
                    ?cmd
                );
                let cmd = self.parse_into_command(cmd)?.spawn().await?;
                let exit_status = cmd.wait().await?;
                Self::handle_status_errors(exit_status)?;
                Ok(())
            }
        }
    }

    fn status_from_num(num: u8, task: ServerCommands) -> bool {
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
        self.cmd_num(
            db_credentials::search_command().collect(),
            ServerCommands::Scan,
        )
        .await
    }

    pub async fn start(&self, args: CommandSpawnOptions) -> Result<()> {
        self.cmd_persistent(db_credentials::start_command().collect(), args)
            .await
    }

    pub async fn clean(&self) -> Result<()> {
        self.cmd(db_credentials::clean_command().collect())
            .await
            .wrap_err("Couldn't clean surreal server")?;
        Ok(())
    }

    pub async fn kill(&self) -> Result<bool> {
        self.cmd_num(
            db_credentials::kill_command().collect(),
            ServerCommands::Kill,
        )
        .await
    }
}
