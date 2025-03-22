use crate::prelude::*;

pub trait ServerPath: Sized {
  fn get_dir(&self) -> &RemoteDir;
  fn into_dir(self) -> RemoteDir {
    self.get_dir().clone()
  }
  fn get_path(&self) -> Utf8PathBuf {
    self.get_dir().clone().into_path()
  }

  fn file(&self, path: impl AsRef<str>) -> RemoteFile {
    self.get_dir().join_file_unchecked(path)
  }

  fn dir(&self, path: impl AsRef<str>) -> RemoteDir {
    self.get_dir().join_unchecked(path)
  }
}

#[derive(Clone)]
pub struct Root(RemoteDir);

impl ServerPath for Root {
  fn get_dir(&self) -> &RemoteDir {
    &self.0
  }
}

impl Root {
  pub fn new() -> Self {
    Self(RemoteDir::new_unchecked("/home/ah/jyd".into()))
  }
}
