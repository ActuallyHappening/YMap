use std::fs;

use crate::prelude::*;
use ::grass;
use utils::{
  paths::{FileExists, PathWrapper},
  project_paths::project,
};

pub struct GrassArgs {
  main_file: FileExists,
  output_file: FileExists,
}

impl GrassArgs {
  pub fn default(paths: project::Root) -> Result<Self> {
    let main_file = paths.get_dir().dir("style")?.file("main.scss")?;
    let output_file = paths
      .get_dir()
      .join("target")
      .join("site")
      .join("pkg")
      .join("jyd-website.css")
      .check_file_exists()?;
    Ok(Self {
      main_file,
      output_file,
    })
  }
}

pub fn compile() -> Result<()> {
  let paths = project::Root::new()?;
  let args = GrassArgs::default(paths)?;
  let res = grass::from_path(args.main_file.as_path(), &grass::Options::default())?;
  fs::write(&args.output_file, res)?;

  info!(
    "Compiled {} using grass to {}",
    args.main_file, args.output_file
  );

  Ok(())
}
