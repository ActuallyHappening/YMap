use crate::prelude::*;

pub type Result<T> = core::result::Result<T, PathsError>;

pub use local::*;
pub mod local;

pub use remote::*;
pub mod remote;

pub trait PathWrapper {
  fn into_path(self) -> Utf8PathBuf;
  fn as_path(&self) -> &Utf8Path;
  fn represents_dir(&self) -> bool {
    self.as_path().is_dir()
  }
  fn represents_file(&self) -> bool {
    !self.represents_dir()
  }

  fn as_str(&self) -> &str {
    self.as_path().as_str()
  }
  /// Uses [`PathWrapper::into_path`], not the types
  /// [`std::fmt::Display`] impl
  fn path_to_string(&self) -> String {
    self.as_path().to_string()
  }
}

macro_rules! path_wrapper {
  ($ty:ty) => {
    impl Deref for $ty {
      type Target = Utf8Path;

      fn deref(&self) -> &Self::Target {
        self.as_path()
      }
    }

    impl Display for $ty {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_path())
      }
    }

    impl AsRef<Utf8Path> for $ty {
      fn as_ref(&self) -> &Utf8Path {
        self.as_path()
      }
    }

    impl From<$ty> for Utf8PathBuf {
      fn from(path: $ty) -> Self {
        path.into_path()
      }
    }

    impl AsRef<std::path::Path> for $ty {
      fn as_ref(&self) -> &std::path::Path {
        self.as_path().as_ref()
      }
    }

    impl From<$ty> for std::path::PathBuf {
      fn from(path: $ty) -> Self {
        path.into_path().into()
      }
    }

    impl AsRef<std::ffi::OsStr> for $ty {
      fn as_ref(&self) -> &std::ffi::OsStr {
        self.as_path().as_ref()
      }
    }
  };
}
pub(crate) use path_wrapper;

#[extension(pub trait PathsExt)]
impl Utf8PathBuf {
  fn check_dir_exists(self) -> Result<DirExists> {
    DirExists::try_from(self)
  }

  fn check_file_exists(self) -> Result<FileExists> {
    FileExists::try_from(self)
  }
}

impl PathsExt for std::path::PathBuf {
  fn check_dir_exists(self) -> Result<DirExists> {
    DirExists::try_from(
      Utf8PathBuf::try_from(self.clone())
        .map_err(|err| PathsError::NotUtf8PathBuf { path: self, err })?,
    )
  }

  fn check_file_exists(self) -> Result<FileExists> {
    FileExists::try_from(
      Utf8PathBuf::try_from(self.clone())
        .map_err(|err| PathsError::NotUtf8PathBuf { path: self, err })?,
    )
  }
}

#[derive(Debug, thiserror::Error)]
pub enum PathsError {
  #[error("The path {path} is not a valid directory")]
  NotDir { path: Utf8PathBuf },

  #[error("The path {path} is not a valid file")]
  NotFile { path: Utf8PathBuf },

  #[error("The path {path} is not valid Utf8: {err}")]
  NotUtf8PathBuf {
    path: std::path::PathBuf,
    err: camino::FromPathBufError,
  },
}
