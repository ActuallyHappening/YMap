pub(crate) use crate::error::WrapReportedErr as _;
pub(crate) use crate::io::MapIoError as _;

pub(crate) use color_eyre::Report;
pub(crate) use std::sync::Arc;

// public internal deps
pub use crate::assert::{
	eyre_assert, eyre_assert as assert, eyre_assert_eq, eyre_assert_eq as assert_eq,
};
pub use crate::path::{Utf8Path, Utf8PathBuf};
pub use crate::{env, fs, io, path};

// public external deps
pub use color_eyre::eyre::{WrapErr as _, bail, eyre};
pub use extension_traits::extension;
pub use tracing::{debug, error, info, trace, warn};
