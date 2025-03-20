use utils::{
  cmds::{self, IntoRemoteArgs, ssh},
  paths::{RemoteDir, RemoteFile},
  tools::rsync,
};

use crate::{
  args::BuildOutput,
  paths::{Prod, Stage},
  prelude::*,
};

/// Represents that the staging is updated
pub struct PublishedToStageKey(());

impl PublishedToStageKey {
  pub fn assume_already_staged() -> Self {
    PublishedToStageKey(())
  }
}

/// Builds locally, puts onto server staging
pub fn copy_leptos_build_to_staging(server: &ssh::Session, paths: &BuildOutput) -> Result<()> {
  use filesize::PathExt as _;
  let local_binary = paths.server_binary()?;
  let bytes = std::path::PathBuf::from(local_binary.as_path())
    .size_on_disk()
    .wrap_err("Couldn't get size of server binary")?;
  let binary_size = size::Size::from_bytes(bytes);

  debug!(%binary_size, "Copying the local build to staging on the server ...");

  // copy site dir
  let args = rsync::RSyncArgs {
    session: server,
    from: paths.site()?.into(),
    to: server.paths().stage().into_dir().into(),
  };
  cmds::local::Command::pure(args)?.run_and_wait()?;

  // copy server binary
  let args = rsync::RSyncArgs {
    session: server,
    from: paths.server_binary()?.into(),
    to: server.paths().stage().server_binary().into(),
  };
  cmds::local::Command::pure(args)?.run_and_wait()?;

  info!(
    message = "Copied the local build to staging on the server",
    note = "You'll want to actually start the server",
  );

  Ok(())
}

pub async fn copy_stage_to_prod(
  server: &ssh::Session,
  stage: &Stage,
  prod: &Prod,
  _key: PublishedToStageKey,
) -> Result<()> {
  /// rm -rf path
  #[derive(Clone)]
  struct RmRFArgs {
    path: RemoteDir,
  }

  impl IntoRemoteArgs for RmRFArgs {
    fn binary_name() -> String {
      "rm".into()
    }
    fn binary_path(&self) -> Result<RemoteFile> {
      // avoids using nu builtin
      Ok(RemoteFile::new_unchecked(Utf8PathBuf::from("/usr/bin/rm")))
    }
    fn into_args(self) -> Vec<String> {
      vec!["-rf".into(), self.path.to_string()]
    }
  }

  #[derive(Clone)]
  struct CpArgs {
    from: RemoteDir,
    to: RemoteDir,
  }

  impl IntoRemoteArgs for CpArgs {
    fn binary_name() -> String {
      "cp".into()
    }
    fn binary_path(&self) -> Result<RemoteFile> {
      // avoids using nu builtin
      Ok(RemoteFile::new_unchecked(Utf8PathBuf::from("/usr/bin/cp")))
    }
    fn into_args(self) -> Vec<String> {
      vec![
        "-f".into(),
        "-R".to_string(),
        self.from.to_string(),
        self.to.to_string(),
      ]
    }
  }

  server
    .command(RmRFArgs {
      path: prod.get_dir().clone(),
    })?
    .run_and_wait()
    .await?;

  let args = CpArgs {
    from: stage.get_dir().clone(),
    to: prod.get_dir().clone(),
  };
  server.command(args)?.run_and_wait().await?;

  info!(
    "Copied staging dir {} to prod dir {}",
    stage.get_dir(),
    prod.get_dir()
  );

  Ok(())
}
