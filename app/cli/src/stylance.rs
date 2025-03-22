use crate::prelude::*;
use utils::{
  cmds::{self, IntoArgs},
  paths::DirExists,
  project_paths::project::{self, Root},
};

pub struct StylanceArgs {
  manifest_path: DirExists,
  watch: bool,
}

impl StylanceArgs {
  pub fn build_once(paths: &Root) -> Result<Self> {
    Ok(Self {
      manifest_path: paths.dir("app")?.clone(),
      watch: false,
    })
  }

  #[allow(dead_code)]
  pub fn watch(paths: &Root) -> Result<Self> {
    Ok(Self {
      manifest_path: paths.dir("app")?.clone(),
      watch: true,
    })
  }

  fn into_library_linked_config(self) -> Result<stylance_core::Config> {
    let config = stylance_core::load_config(self.manifest_path.as_ref())?;
    Ok(config)
  }
}

impl IntoArgs for StylanceArgs {
  fn binary_name() -> String {
    "stylance".into()
  }
  fn installation_suggestion() -> Option<String> {
    Some("cargo install stylance-cli".into())
  }

  fn into_args(self) -> Vec<String> {
    let mut ret = vec![self.manifest_path.to_string()];
    if self.watch {
      ret.push("--watch".into());
    }
    ret
  }
}

pub fn build_once() -> Result<()> {
  let paths = project::Root::new()?;
  let args = StylanceArgs::build_once(&paths)?;

  // cmds::local::Command::pure(args)?.run_and_wait()
  stylance_cli::run(
    args.manifest_path.clone().as_ref(),
    &args.into_library_linked_config()?,
  )
}

/// TODO: use library linked
/// https://docs.rs/notify/latest/notify/
#[allow(dead_code)]
pub fn watch() -> Result<()> {
  let paths = project::Root::new()?;
  let args = StylanceArgs::watch(&paths)?;

  cmds::local::Command::pure(args)?.run_and_wait()
}
