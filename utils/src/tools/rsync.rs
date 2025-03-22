use crate::paths::{DirExists, FileExists};
use crate::prelude::*;

use crate::{
  cmds::{
    IntoArgs,
    ssh::{self},
  },
  paths::{LocalPath, PathWrapper, RemoteDir, RemoteFile, RemotePath},
};

pub enum LocalOrRemotePath {
  Local(LocalPath),
  Remote(RemotePath),
}

crate::paths::path_wrapper!(LocalOrRemotePath);

impl PathWrapper for LocalOrRemotePath {
  fn into_path(self) -> Utf8PathBuf {
    match self {
      Self::Local(local) => local.into_path(),
      Self::Remote(remote) => remote.into_path(),
    }
  }
  fn as_path(&self) -> &Utf8Path {
    match self {
      Self::Local(local) => local.as_path(),
      Self::Remote(remote) => remote.as_path(),
    }
  }
  fn represents_dir(&self) -> bool {
    match self {
      Self::Local(local) => local.represents_dir(),
      Self::Remote(remote) => remote.represents_dir(),
    }
  }
}

impl LocalOrRemotePath {
  pub fn is_local(&self) -> bool {
    matches!(self, LocalOrRemotePath::Local(_))
  }

  pub fn is_remote(&self) -> bool {
    matches!(self, LocalOrRemotePath::Remote(_))
  }
}

impl From<FileExists> for LocalOrRemotePath {
  fn from(file: FileExists) -> Self {
    Self::Local(LocalPath::File(file))
  }
}

impl From<DirExists> for LocalOrRemotePath {
  fn from(dir: DirExists) -> Self {
    Self::Local(LocalPath::Dir(dir))
  }
}

impl From<RemoteDir> for LocalOrRemotePath {
  fn from(dir: RemoteDir) -> Self {
    Self::Remote(dir.into())
  }
}

impl From<RemoteFile> for LocalOrRemotePath {
  fn from(file: RemoteFile) -> Self {
    Self::Remote(file.into())
  }
}

pub struct RSyncArgs<'s> {
  pub from: LocalOrRemotePath,
  /// If copying a dir, don't include the final name here
  /// as it will be copied 'into' the remote_path dir
  pub to: LocalOrRemotePath,
  pub session: &'s ssh::Session,
}

impl IntoArgs for RSyncArgs<'_> {
  fn binary_name() -> String {
    "rsync".into()
  }

  fn into_args(self) -> Vec<String> {
    let mut ret = vec!["--safe-links".into()];
    if self.from.represents_dir() {
      ret.push("--recursive".into());
    }

    let ssh_name = self.session.ssh_name();
    ret.extend([
      self.from.into_path().to_string(),
      format!("{}:{}", ssh_name, self.to),
    ]);

    ret
  }
}
