use target::Target;

use crate::{
  paths::{DirExists, FileExists, PathsError},
  prelude::*,
};

pub trait ProjectPath: Sized {
  fn get_dir(&self) -> &DirExists;

  fn file(&self, file: impl AsRef<str>) -> Result<FileExists> {
    Ok(self.get_dir().file(file)?)
  }
  fn dir(&self, dir: impl AsRef<str>) -> Result<DirExists> {
    Ok(self.get_dir().dir(dir)?)
  }
}

pub struct Root(DirExists);

impl ProjectPath for Root {
  fn get_dir(&self) -> &DirExists {
    &self.0
  }
}

impl Root {
  pub fn new() -> Result<Root> {
    let root = Utf8PathBuf::from("/home/ah/Desktop/JYD/").check_dir_exists()?;
    Ok(Root(root))
  }

  pub fn manifest_file(&self) -> Result<FileExists> {
    self.file("Cargo.toml")
  }

  pub fn style_dir(&self) -> Result<DirExists> {
    self.dir("style")
  }

  pub fn app_component(&self, component_name: impl AsRef<str>) -> Result<DirExists> {
    Ok(
      self
        .dir("app")?
        .join("src")
        .join("components")
        .join(component_name.as_ref())
        .check_dir_exists()?,
    )
  }
}

pub mod target {
  use crate::{
    paths::{DirExists, FileExists},
    prelude::*,
  };

  use super::{ProjectPath, Root};

  #[derive(Clone)]
  pub struct Target(DirExists);

  impl ProjectPath for Target {
    fn get_dir(&self) -> &DirExists {
      &self.0
    }
  }

  impl Root {
    pub fn target(&self) -> Result<Target> {
      Ok(Target(self.dir("target")?))
    }
  }

  pub enum BinLocation {
    Debug,
    Release,
  }

  impl Target {
    pub fn bin(&self, location: &BinLocation, binary_name: impl AsRef<str>) -> Result<FileExists> {
      let dir = match location {
        BinLocation::Debug => self.dir("debug")?,
        BinLocation::Release => self.dir("release")?,
      };
      Ok(dir.file(binary_name)?)
    }
  }
}
