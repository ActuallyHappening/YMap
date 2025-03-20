use pueue::{ProcInfo, TaskHandle};
use utils::project_paths::server;

use crate::{args::RemoteBinaryRunner, prelude::*};

const WEBSITE_LABEL: &str = "website";

pub struct Stage(pub(crate) RemoteBinaryRunner);

impl ProcInfo for Stage {
  fn group() -> &'static str {
    pueue::STAGE_GROUP
  }
  fn label() -> &'static str {
    WEBSITE_LABEL
  }
}

pub struct Prod(pub(crate) RemoteBinaryRunner);

impl ProcInfo for Prod {
  fn group() -> &'static str {
    pueue::PROD_GROUP
  }
  fn label() -> &'static str {
    WEBSITE_LABEL
  }
}

#[extension(pub trait PueueExt)]
impl pueue::Session {
  fn staging(&mut self, path: server::Root) -> TaskHandle<'_, Stage> {
    TaskHandle::new(self, RemoteBinaryRunner::staged(path))
  }

  fn prod(&mut self, path: server::Root) -> TaskHandle<'_, Prod> {
    TaskHandle::new(self, RemoteBinaryRunner::prod(path))
  }
}
