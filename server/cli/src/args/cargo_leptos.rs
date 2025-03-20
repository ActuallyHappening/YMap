use std::collections::HashMap;

use utils::{
  cmds::{self, IntoArgs as _},
  paths::FileExists,
  project_paths::project::{self, Root},
};

use crate::prelude::*;

use super::BuildOutput;

#[derive(Debug)]
pub(crate) struct CargoLeptosArgs {
  pub release_level: ReleaseLevel,
  pub cmd: Cmd,
  manifest_path: FileExists,
  extra_args: Vec<String>,
}

/// Lower variants imply variants above it (after dev)
/// e.g. [`ReleaseLevel::ProdOnServer`] implies [`ReleaseLevel::Release`]
/// and [`ReleaseLevel::PreCompress`]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Default, clap::ValueEnum, Debug)]
pub enum ReleaseLevel {
  #[default]
  Dev,
  Release,
  PreCompress,
  ProdOnServer,
}

impl ReleaseLevel {
  pub fn is_release(self) -> bool {
    self >= ReleaseLevel::Release
  }

  pub fn should_precompress(self) -> bool {
    self >= ReleaseLevel::PreCompress
  }

  pub fn prod_on_server(self) -> bool {
    self >= ReleaseLevel::ProdOnServer
  }

  pub(crate) fn include_env_vars(self) -> bool {
    self.prod_on_server()
  }
}

impl CargoLeptosArgs {
  pub fn serve(paths: &project::Root, extra_args: Vec<String>) -> Result<Self> {
    Ok(Self {
      manifest_path: paths.manifest_file()?,
      cmd: Cmd::Serve,
      release_level: ReleaseLevel::default(),
      extra_args,
    })
  }

  pub fn build(paths: &project::Root, extra_args: Vec<String>) -> Result<Self> {
    Ok(Self {
      manifest_path: paths.manifest_file()?,
      cmd: Cmd::Build,
      release_level: ReleaseLevel::default(),
      extra_args,
    })
  }

  pub fn with_release(mut self, level: ReleaseLevel) -> Self {
    self.release_level = level;
    self
  }

  pub(crate) fn into_library_linked_cli(self) -> cargo_leptos::config::Cli {
    cargo_leptos::config::Cli::parse_from(self.into_args())
  }
}

#[derive(Debug)]
pub enum Cmd {
  Serve,
  Build,
}

impl Display for Cmd {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Cmd::Serve => write!(f, "serve"),
      Cmd::Build => write!(f, "build"),
    }
  }
}

impl cmds::IntoArgs for CargoLeptosArgs {
  fn binary_name() -> String {
    "cargo-leptos".into()
  }
  fn installation_suggestion() -> Option<String> {
    Some("This should be library linked, or `cargo install cargo-leptos`".into())
  }

  /// Puts through the `CARGO` env var.
  /// Not sure if this does anything
  fn env_vars(&self) -> HashMap<Box<str>, Option<String>> {
    let mut map = HashMap::new();
    map.insert("CARGO".into(), None);
    map
  }

  fn into_args(self) -> Vec<String> {
    let mut ret = vec![
      "cargo-leptos: library linked".into(),
      format!("--manifest-path={}", self.manifest_path),
      // "--log=server".into(),
      self.cmd.to_string(),
      "-vv".into(), // handled by xtask/main.rs fn install_tracing
    ];

    if self.release_level.is_release() {
      ret.push("--release".into());
    }
    if self.release_level.should_precompress() {
      ret.push("--precompress".into());
    }
    if self.release_level.prod_on_server() {
      debug!("Building for prod-on-server");
      ret.push("--features=prod".into());
    }
    ret.extend(self.extra_args);

    ret
  }
}

impl CargoLeptosArgs {
  async fn run(self) -> Result<()> {
    let inner = self.into_library_linked_cli();
    debug!(args =?inner, "Running library-linked cargo leptos");
    cargo_leptos::run(inner)
      .await
      .wrap_err("Cargo leptos (library linked) failed")
  }
}

pub async fn dev_serve(extra_args: Vec<String>) -> Result<()> {
  let paths = project::Root::new()?;
  let args = CargoLeptosArgs::serve(&paths, extra_args)?;

  // SAFETY: this is not safe but who cares lol
  unsafe { std::env::set_var("JYD_DEV_SERVE", "pls") };
  debug!("Passing `JYD_DEV_SERVE` env var to server");

  args.run().await.wrap_err("Couldn't build server for dev")
}

pub async fn prod_build(level: ReleaseLevel) -> Result<BuildOutput> {
  let paths = project::Root::new()?;
  let args = CargoLeptosArgs::build(&paths, vec![])?.with_release(level);

  info!(message = "Building project using cargo-leptos", ?args);

  if level.include_env_vars() {
    #[allow(unused_mut)]
    let mut envs = Vec::from([
      ("JYD_STRIPE_PUBLISH_KEY", env::stripe::LIVE_PUBLISH_KEY),
      ("JYD_STRIPE_TEST_PUBLISH_KEY", env::stripe::TEST_PUBLISH_KEY),
      ("JYD_STRIPE_TEST_API_KEY", env::stripe::TEST_API_KEY),
      ("JYD_DB_ROOT_PASS", env::db::ROOT_PASS),
    ]);
    #[cfg(feature = "prod")]
    envs.push(("JYD_STRIPE_API_KEY", env::stripe::LIVE_API_KEY));
    for (key, value) in envs.iter() {
      unsafe { std::env::set_var(key, value) };
    }
  }

  args.run().await?;

  Ok(paths.target()?.build_output(level))
}
