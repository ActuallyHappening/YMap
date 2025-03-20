use utils::{
  paths::{RemoteDir, RemoteFile},
  project_paths::server,
};

use crate::prelude::*;

pub mod certs;

pub trait WebsiteRoot: ServerPath {
  fn server_binary(&self) -> RemoteFile {
    self.file("server")
  }

  fn site(&self) -> RemoteDir {
    self.dir("site")
  }
}

/// website
pub struct Stage(RemoteDir);

impl ServerPath for Stage {
  fn get_dir(&self) -> &RemoteDir {
    &self.0
  }
}
impl WebsiteRoot for Stage {}

/// prod-website
pub struct Prod(RemoteDir);

impl ServerPath for Prod {
  fn get_dir(&self) -> &RemoteDir {
    &self.0
  }
}

impl WebsiteRoot for Prod {}

#[extension(pub trait ServerPathExt)]
impl server::Root {
  fn stage(&self) -> Stage {
    Stage(self.dir("website-stage"))
  }

  fn prod(&self) -> Prod {
    Prod(self.dir("website-prod"))
  }
}
