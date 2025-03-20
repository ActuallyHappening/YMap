use std::{
  collections::HashMap,
  net::{Ipv4Addr, SocketAddrV4},
};

use utils::{
  cmds::{self, IntoRemoteArgs},
  paths::RemoteFile,
  project_paths::server,
};

use crate::{paths::certs::LetsEncryptCertsDir, prelude::*, spueue};

pub use local::*;
pub mod local;

pub mod cargo_leptos;

struct GenericBinaryArgs {
  /// If [`None`], doesn't add certs
  pub env_certs_dir: Option<Utf8PathBuf>,
  pub site_root_dir: Utf8PathBuf,
  pub bind_addr: SocketAddrV4,
}

impl GenericBinaryArgs {
  pub fn sourced_envs(&self) -> HashMap<Box<str>, String> {
    let mut map = HashMap::new();
    if let Some(env_certs_dir) = &self.env_certs_dir {
      map.insert("JYD_CERTS".into(), env_certs_dir.to_string());
    }
    map.insert("LEPTOS_SITE_ROOT".into(), self.site_root_dir.to_string());
    map.insert("LEPTOS_SITE_ADDR".into(), self.bind_addr.to_string());
    map
  }
}

pub struct RemoteBinaryRunner {
  server_binary: RemoteFile,
  args: GenericBinaryArgs,
}

impl IntoRemoteArgs for RemoteBinaryRunner {
  fn binary_name() -> String {
    "server".into()
  }
  fn installation_suggestion() -> Option<String> {
    Some("cargo x build --level prod-on-server; cargo x copy server --level prod-on-server".into())
  }
  fn binary_path(&self) -> Result<RemoteFile> {
    Ok(self.server_binary.clone())
  }
  fn included_env_vars(&self) -> Result<HashMap<Box<str>, String>> {
    Ok(self.args.sourced_envs())
  }

  fn into_args(self) -> Vec<String> {
    vec![]
  }
}

impl RemoteBinaryRunner {
  pub fn staged(paths: server::Root) -> spueue::Stage {
    let bind_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 12369);
    spueue::Stage(Self {
      args: GenericBinaryArgs {
        env_certs_dir: Some(LetsEncryptCertsDir::new().get_path()),
        site_root_dir: paths.stage().site().into_path(),
        bind_addr,
      },
      server_binary: paths.stage().server_binary(),
    })
  }

  pub fn prod(paths: server::Root) -> spueue::Prod {
    let bind_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 443);
    spueue::Prod(Self {
      args: GenericBinaryArgs {
        env_certs_dir: Some(LetsEncryptCertsDir::new().get_path()),
        site_root_dir: paths.prod().site().into_path(),
        bind_addr,
      },
      server_binary: paths.prod().server_binary(),
    })
  }
}

impl IntoRemoteArgs for spueue::Stage {
  fn binary_name() -> String {
    <RemoteBinaryRunner as IntoRemoteArgs>::binary_name()
  }
  fn installation_suggestion() -> Option<String> {
    <RemoteBinaryRunner as IntoRemoteArgs>::installation_suggestion()
  }
  fn binary_path(&self) -> Result<RemoteFile> {
    cmds::IntoRemoteArgs::binary_path(&self.0)
  }
  fn included_env_vars(&self) -> Result<HashMap<Box<str>, String>> {
    cmds::IntoRemoteArgs::included_env_vars(&self.0)
  }
  fn into_args(self) -> Vec<String> {
    cmds::IntoRemoteArgs::into_args(self.0)
  }
}
impl IntoRemoteArgs for spueue::Prod {
  fn binary_name() -> String {
    <RemoteBinaryRunner as IntoRemoteArgs>::binary_name()
  }
  fn installation_suggestion() -> Option<String> {
    <RemoteBinaryRunner as IntoRemoteArgs>::installation_suggestion()
  }
  fn binary_path(&self) -> Result<RemoteFile> {
    cmds::IntoRemoteArgs::binary_path(&self.0)
  }
  fn included_env_vars(&self) -> Result<HashMap<Box<str>, String>> {
    cmds::IntoRemoteArgs::included_env_vars(&self.0)
  }
  fn into_args(self) -> Vec<String> {
    cmds::IntoRemoteArgs::into_args(self.0)
  }
}
