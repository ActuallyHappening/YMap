use std::str::FromStr;

use super::{PathWrapper, PathsError, Result, path_wrapper};
use crate::prelude::*;

pub enum LocalPath {
  Dir(DirExists),
  File(FileExists),
}

impl PathWrapper for LocalPath {
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

impl From<DirExists> for LocalPath {
  fn from(dir: DirExists) -> Self {
    Self::Dir(dir)
  }
}

impl From<FileExists> for LocalPath {
  fn from(file: FileExists) -> Self {
    Self::File(file)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirExists(Utf8PathBuf);

path_wrapper!(DirExists);

impl PathWrapper for DirExists {
  fn into_path(self) -> Utf8PathBuf {
    self.0
  }

  fn as_path(&self) -> &Utf8Path {
    self.0.as_path()
  }
}

impl DirExists {
  pub fn file(&self, file: impl AsRef<str>) -> Result<FileExists> {
    self.join(file.as_ref()).check_file_exists()
  }

  pub fn dir(&self, dir: impl AsRef<str>) -> Result<DirExists> {
    self.join(dir.as_ref()).check_dir_exists()
  }
}

impl TryFrom<Utf8PathBuf> for DirExists {
  type Error = PathsError;

  fn try_from(value: Utf8PathBuf) -> Result<Self> {
    if value.is_dir() {
      Ok(DirExists(value))
    } else {
      Err(PathsError::NotDir { path: value })
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileExists(Utf8PathBuf);

path_wrapper!(FileExists);

impl PathWrapper for FileExists {
  fn into_path(self) -> Utf8PathBuf {
    self.0
  }

  fn as_path(&self) -> &Utf8Path {
    self.0.as_path()
  }
}

impl TryFrom<Utf8PathBuf> for FileExists {
  type Error = PathsError;

  fn try_from(value: Utf8PathBuf) -> Result<Self> {
    if value.is_file() {
      Ok(FileExists(value))
    } else {
      Err(PathsError::NotFile { path: value })
    }
  }
}

impl FromStr for FileExists {
  type Err = color_eyre::Report;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    Utf8PathBuf::from(s)
      .check_file_exists()
      .wrap_err("Couldn't confirm that the file exists")
  }
}
