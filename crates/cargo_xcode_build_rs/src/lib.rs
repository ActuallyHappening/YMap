//! <h1 class="warning">
//! Warning: This library is unstable, only the executable's behaviour is subject to semver
//! </h1>
#![doc = include_str!("../README.md")]

pub mod prelude {
	pub(crate) use std::env;

	pub(crate) use camino::{Utf8Path, Utf8PathBuf};
	pub(crate) use color_eyre::{
		eyre::{eyre, Context as _, Report},
		Section as _,
	};
	pub(crate) use serde::Deserialize;
	#[allow(unused_imports)]
	pub(crate) use tracing::{debug, error, info, trace, warn};

	pub use crate::cli::*;
	pub use crate::config::*;
	pub use crate::helpers::*;
}
mod cli;

mod config;

mod helpers;
