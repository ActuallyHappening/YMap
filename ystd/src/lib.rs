pub mod prelude {
	pub use crate::path::{Path, PathBuf, Utf8Path, Utf8PathBuf, YPath, YPathBuf};
	pub(crate) use color_eyre::Report;
	pub(crate) use color_eyre::eyre::{WrapErr as _, bail, eyre};
	pub(crate) use std::sync::Arc;
	pub use tracing::{debug, error, info, trace, warn};
}

pub mod path;
pub mod io;
pub mod fs;
pub mod env;
