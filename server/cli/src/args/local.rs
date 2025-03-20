use std::{collections::HashMap, net::SocketAddrV4};

use utils::{
  cmds::{self, BinaryPath},
  paths::{DirExists, FileExists},
  project_paths::project::target::{self, Target},
};

use crate::prelude::*;

use super::{GenericBinaryArgs, cargo_leptos::ReleaseLevel};

pub(crate) struct BuildOutput {
  target: target::Target,
  location: target::BinLocation,
}

impl Deref for BuildOutput {
  type Target = target::Target;

  fn deref(&self) -> &Self::Target {
    &self.target
  }
}

#[extension(pub(crate) trait TargetPaths)]
impl Target {
  fn build_output(&self, level: ReleaseLevel) -> BuildOutput {
    BuildOutput {
      target: self.clone(),
      location: level.bin_location(),
    }
  }

  fn site(&self) -> Result<DirExists> {
    self.dir("site")
  }
}

impl BuildOutput {
  pub fn server_binary(&self) -> Result<FileExists> {
    self.bin(&self.location, "server")
  }
}

impl ReleaseLevel {
  fn bin_location(self) -> target::BinLocation {
    match self {
      Self::Dev => target::BinLocation::Debug,
      Self::Release | Self::PreCompress | Self::ProdOnServer => target::BinLocation::Release,
    }
  }

  fn build_output(self, target: target::Target) -> BuildOutput {
    BuildOutput {
      target,
      location: self.bin_location(),
    }
  }
}

pub struct LocalBinaryRunner {
  server_binary: FileExists,
  args: GenericBinaryArgs,
}

impl cmds::IntoArgs for LocalBinaryRunner {
  fn binary_name() -> String {
    "server".into()
  }
  fn installation_suggestion() -> Option<String> {
    Some("Build cargo-leptos, library internal".into())
  }
  fn binary_path(&self) -> Result<BinaryPath> {
    Ok(BinaryPath::Local(self.server_binary.clone()))
  }
  fn included_env_vars(&self) -> Result<HashMap<Box<str>, String>> {
    Ok(self.args.sourced_envs())
  }

  fn into_args(self) -> Vec<String> {
    vec![]
  }
}

impl LocalBinaryRunner {
  pub fn new(paths: &BuildOutput, socket: SocketAddrV4) -> Result<Self> {
    Ok(Self {
      args: GenericBinaryArgs {
        env_certs_dir: None,
        site_root_dir: paths.site()?.as_path().to_owned(),
        bind_addr: socket,
      },
      server_binary: paths.server_binary()?,
    })
  }
}
