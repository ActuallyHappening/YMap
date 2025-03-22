use crate::{prelude::*, project_paths::server::Root};

use super::{IntoRemoteArgs, NuCommand};
use crate::cmds::IntoArgs;

/// Cheap to clone
pub struct Session {
  session: openssh::Session,
  ssh_name: String,
  paths: Root,
}

impl Session {
  pub async fn new() -> Result<Self> {
    let ssh_name = env::server::SSH_NAME.into();
    let session = openssh::Session::connect_mux(&ssh_name, openssh::KnownHosts::Strict)
      .await
      .wrap_err("Couldn't connect to server using (native mux) ssh")?;

    // let session = openssh::SessionBuilder::default()
    //   .user("root".into())
    //   .known_hosts_check(openssh::KnownHosts::Strict)
    //   .keyfile("/home/ah/.env/jyd/server")
    //   .connect_mux("119.42.55.228")
    //   .await
    //   .wrap_err("Couldn't connect to server using (native mux) openssh lib")?;

    session.command("whoami").spawn().await?.wait().await?;

    Ok(Self {
      ssh_name,
      paths: Root::new(),
      session,
    })
  }

  /// `Clone` is to allow for debuggability
  pub fn command<A: IntoRemoteArgs + Clone>(&self, args: A) -> Result<Command<'_>> {
    let binary_path = args.binary_path()?;
    let env_and_cmd = {
      let mut raw_cmd = String::new();
      let envs = args.included_env_vars()?;
      for (key, value) in envs {
        raw_cmd.push_str(&key);
        raw_cmd.push('=');
        raw_cmd.push_str(&value);
        raw_cmd.push(' ');
      }
      raw_cmd.push_str(binary_path.as_str());
      raw_cmd
    };
    let mut cmd = self.session.raw_command(env_and_cmd);
    let debug = args.clone().resolve()?;

    if args.raw_args() {
      cmd.raw_args(args.into_args());
    } else {
      cmd.args(args.into_args());
    }

    Ok(Command { inner: cmd, debug })
  }

  pub fn nu_command<A: NuCommand>(&self, args: A) -> Result<Command<'_>> {
    // todo: extract this maybe?
    let binary_path = "/usr/bin/nu";

    let command = args.command();
    let mut cmd = self.session.command(binary_path);
    cmd.arg("-c");
    // shell escaping go brr
    cmd.raw_arg(format!("\"{}\"", command));

    Ok(Command {
      inner: cmd,
      debug: command,
    })
  }

  pub fn ssh_name(&self) -> &str {
    &self.ssh_name
  }

  pub fn paths(&self) -> Root {
    self.paths.clone()
  }
}

pub struct Command<'s> {
  inner: openssh::OwningCommand<&'s openssh::Session>,
  debug: String,
}

impl Command<'_> {
  pub async fn run_and_wait(mut self) -> Result<()> {
    info!("Running command on server over ssh: {:?}", self.debug);
    self.inner.spawn().await?.wait().await?;
    Ok(())
  }
}
