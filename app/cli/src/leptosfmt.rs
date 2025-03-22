use crate::prelude::*;
use utils::{
  cmds::{self, IntoArgs},
  paths::{DirExists, FileExists},
  project_paths::project::{self, Root},
};

pub struct Args {
  pub input: DirExists,
  pub config_file: FileExists,
}

impl Args {
  pub fn default(paths: &Root) -> Result<Self> {
    let input = paths.dir("app")?.dir("src")?;
    let config = paths.file("rustfmt.toml")?;
    Ok(Self {
      input,
      config_file: config,
    })
  }
}

impl IntoArgs for Args {
  fn binary_name() -> String {
    "leptosfmt".to_string()
  }

  fn installation_suggestion() -> Option<String> {
    Some("cargo install leptosfmt".to_string())
  }

  fn into_args(self) -> Vec<String> {
    vec![
      self.input.to_string(),
      format!("--config-file={}", self.config_file),
    ]
  }
}

pub fn fmt_subprocess() -> Result<()> {
  let paths = project::Root::new()?;
  let args = Args::default(&paths)?;
  cmds::local::Command::pure(args)?.run_and_wait()
}

/// Idea: link to `leptosfmt-formatter` crate
/// see
pub fn fmt() -> Result<()> {
  fmt_subprocess()
}
