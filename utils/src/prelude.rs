//! Anything `pub` ends up in `crate::cli_prelude`
pub type Result<T> = core::result::Result<T, color_eyre::Report>;
pub use color_eyre::Section as _;
pub use color_eyre::eyre::{WrapErr as _, bail, eyre};

pub use camino::{Utf8Path, Utf8PathBuf};
pub use extension_traits::extension;
pub use serde::{Deserialize, Serialize};
pub use tracing::{debug, error, info, trace, warn};

pub use std::fmt::Display;
pub use std::ops::{Deref, DerefMut};

pub use crate::paths::{
  DirExists, FileExists, PathWrapper as _, PathsExt as _, RemoteDir, RemoteFile,
};
pub use crate::project_paths::{self, project::ProjectPath, server::ServerPath};

#[cfg(feature = "cli-support")]
pub use clap::{self, Parser as _};
pub use color_eyre;
pub use thiserror;
pub use time;
pub use tokio;
pub use tracing;

#[extension(pub trait CustomWrapErr)]
impl<T, E> core::result::Result<T, E>
where
  E: std::fmt::Debug,
{
  fn report_err<D>(self, msg: D) -> Option<T>
  where
    D: Display + Send + Sync + 'static,
  {
    match self {
      Ok(val) => Some(val),
      Err(err) => {
        error!(?err, message = msg.to_string());
        None
      }
    }
  }
}
