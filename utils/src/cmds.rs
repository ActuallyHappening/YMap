use std::collections::HashMap;

use crate::{
  paths::{FileExists, PathWrapper},
  prelude::*,
};

use super::paths::{RemoteFile, path_wrapper};

pub mod local;
pub mod ssh;

pub enum BinaryPath {
  Local(FileExists),
  Remote(RemoteFile),
}
path_wrapper!(BinaryPath);

impl PathWrapper for BinaryPath {
  fn into_path(self) -> Utf8PathBuf {
    match self {
      BinaryPath::Local(file) => file.into_path(),
      BinaryPath::Remote(file) => file.into_path(),
    }
  }
  fn as_path(&self) -> &Utf8Path {
    match self {
      BinaryPath::Local(file) => file.as_path(),
      BinaryPath::Remote(file) => file.as_path(),
    }
  }
}

pub trait IntoArgs: Sized {
  fn binary_name() -> String;
  fn installation_suggestion() -> Option<String> {
    None
  }
  /// Default impl finds [`BinaryPath::Local`] using [`which`]
  fn binary_path(&self) -> Result<BinaryPath> {
    let mut ret = which(Self::binary_name());
    let suggestion = Self::installation_suggestion();
    if let Some(suggestion) = suggestion {
      ret = ret
        .wrap_err("Coulnd't automatically locate local binary")
        .suggestion(suggestion)
    }
    Ok(BinaryPath::Local(ret?))
  }

  /// Should *NOT* be read by users of this trait, as implementors
  /// may chose to instead implement [`IntoArgs::included_env_vars`]
  /// directly
  fn env_vars(&self) -> HashMap<Box<str>, Option<String>> {
    Default::default()
  }

  /// Default reads from [`IntoArgs::env_vars`] and substitutes values from current environment
  fn included_env_vars(&self) -> Result<HashMap<Box<str>, String>> {
    let mut map = HashMap::new();
    for (key, value) in self.env_vars() {
      let key = key.as_ref();
      match value {
        Some(value) => {
          map.insert(key.into(), value);
        }
        None => {
          let value = std::env::var(key).wrap_err(format!(
            "Couldn't find env variable in current environment to substitute into command: {key}"
          ))?;
          map.insert(key.into(), value);
        }
      }
    }
    Ok(map)
  }
  /// Don't override, reads from [`IntoArgs::included_env_vars`]
  fn included_env_vars_string(&self) -> Result<HashMap<String, String>> {
    self
      .included_env_vars()
      .map(|map| map.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
  }

  fn into_args(self) -> Vec<String>;

  /// Resolves into args passable into `nu`.
  /// IDK about escaping tbh
  fn resolve(self) -> Result<String> {
    let mut envs = vec![];

    // envs
    for (key, value) in self.included_env_vars()?.iter() {
      envs.push(format!("{}={}", key, value));
    }

    // args
    let mut args = vec![];
    args.push(self.binary_path()?.into_path().into());
    args.extend(self.into_args());
    let args = args
      .into_iter()
      .map(|arg| shell_escape::escape(arg.into()).to_string())
      .collect::<Vec<String>>();
    envs.extend(args);

    Ok(envs.join(" "))
  }
}

pub trait IntoRemoteArgs {
  fn binary_name() -> String;
  fn installation_suggestion() -> Option<String> {
    None
  }
  fn binary_path(&self) -> Result<RemoteFile>;

  fn included_env_vars(&self) -> Result<HashMap<Box<str>, String>> {
    Ok(Default::default())
  }
  fn included_env_vars_string(&self) -> Result<HashMap<String, String>> {
    self
      .included_env_vars()
      .map(|map| map.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
  }

  /// Whether to escape [`IntoRemoteArgs::raw_args`]
  fn raw_args(&self) -> bool {
    false
  }

  fn into_args(self) -> Vec<String>;
}

impl<T> IntoArgs for T
where
  T: IntoRemoteArgs,
{
  fn binary_name() -> String {
    T::binary_name()
  }

  fn installation_suggestion() -> Option<String> {
    T::installation_suggestion()
  }
  fn binary_path(&self) -> Result<BinaryPath> {
    Ok(BinaryPath::Remote(self.binary_path()?))
  }

  fn env_vars(&self) -> HashMap<Box<str>, Option<String>> {
    unimplemented!("Don't read this")
  }
  fn included_env_vars(&self) -> Result<HashMap<Box<str>, String>> {
    self.included_env_vars()
  }
  fn included_env_vars_string(&self) -> Result<HashMap<String, String>> {
    self.included_env_vars_string()
  }

  fn into_args(self) -> Vec<String> {
    self.into_args()
  }
}

pub trait NuCommand {
  fn command(self) -> String;
}

pub fn which(binary_name: impl AsRef<str>) -> Result<FileExists> {
  which::which(binary_name.as_ref())
    .map_err(|_| eyre!("Couldn't find binary in PATH: {}", binary_name.as_ref()))
    .and_then(|path| {
      Utf8PathBuf::try_from(path)?
        .check_file_exists()
        .map_err(Into::into)
    })
}
