use super::{PathWrapper, path_wrapper};
use crate::prelude::*;

#[derive(Debug)]
pub enum RemotePath {
  Dir(RemoteDir),
  File(RemoteFile),
}

path_wrapper!(RemotePath);

impl PathWrapper for RemotePath {
  fn into_path(self) -> Utf8PathBuf {
    match self {
      Self::Dir(dir) => dir.into_path(),
      Self::File(file) => file.into_path(),
    }
  }
  fn as_path(&self) -> &Utf8Path {
    match self {
      Self::Dir(dir) => dir.as_path(),
      Self::File(file) => file.as_path(),
    }
  }
}

impl From<RemoteDir> for RemotePath {
  fn from(dir: RemoteDir) -> Self {
    Self::Dir(dir)
  }
}

impl From<RemoteFile> for RemotePath {
  fn from(file: RemoteFile) -> Self {
    Self::File(file)
  }
}

#[derive(Debug, Clone)]
pub struct RemoteDir(Utf8PathBuf);

path_wrapper!(RemoteDir);

impl PathWrapper for RemoteDir {
  fn into_path(self) -> Utf8PathBuf {
    self.0
  }

  fn as_path(&self) -> &Utf8Path {
    self.0.as_path()
  }
}

impl RemoteDir {
  pub fn new_unchecked(path: Utf8PathBuf) -> Self {
    Self(path)
  }

  pub fn join_unchecked(&self, path: impl AsRef<str>) -> RemoteDir {
    Self(self.0.join(path.as_ref()))
  }

  pub fn join_file_unchecked(&self, path: impl AsRef<str>) -> RemoteFile {
    RemoteFile::new_unchecked(self.0.join(path.as_ref()))
  }

  pub fn pop_unchecked(&self) -> RemoteDir {
    let mut this = self.0.clone();
    this.pop();
    Self(this)
  }
}

#[derive(Debug, Clone)]
pub struct RemoteFile(Utf8PathBuf);

path_wrapper!(RemoteFile);

impl PathWrapper for RemoteFile {
  fn into_path(self) -> Utf8PathBuf {
    self.0
  }

  fn as_path(&self) -> &Utf8Path {
    self.0.as_path()
  }
}

impl RemoteFile {
  pub fn new_unchecked(path: Utf8PathBuf) -> Self {
    Self(path)
  }

  pub fn join_unchecked(&self, path: impl AsRef<str>) -> Self {
    Self(self.0.join(path.as_ref()))
  }
}
