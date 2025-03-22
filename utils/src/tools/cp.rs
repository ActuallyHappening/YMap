//! Try to avoid using this
use crate::{
  cmds::{self, IntoArgs, IntoRemoteArgs},
  paths::RemoteFile,
  prelude::*,
};

#[derive(Clone)]
pub struct ServerFileArgs {
  pub from: RemoteFile,
  pub to: RemoteFile,
}

impl IntoRemoteArgs for ServerFileArgs {
  fn binary_name() -> String {
    "cp".into()
  }
  fn binary_path(&self) -> Result<RemoteFile> {
    Ok(RemoteFile::new_unchecked("cp".into()))
  }
  fn into_args(self) -> Vec<String> {
    vec![self.from.to_string(), self.to.to_string()]
  }
}

/// Depreciated, todo remove
pub struct CpArgs {
  pub from_dir: Utf8PathBuf,
  pub to_dir_path: Utf8PathBuf,
}

impl IntoArgs for CpArgs {
  fn binary_name() -> String {
    "cp".into()
  }
  fn binary_path(&self) -> Result<cmds::BinaryPath> {
    // uses `nu` builtin
    Ok(cmds::BinaryPath::Remote(RemoteFile::new_unchecked(
      "/usr/bin/cp".into(),
    )))
  }
  fn into_args(self) -> Vec<String> {
    vec![
      "--recursive".into(),
      self.from_dir.to_string(),
      self.to_dir_path.to_string(),
    ]
  }
}
