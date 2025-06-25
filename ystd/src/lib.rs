pub mod prelude {
	pub use crate::assert::{
		eyre_assert, eyre_assert as assert, eyre_assert_eq, eyre_assert_eq as assert_eq,
	};
	pub(crate) use crate::io::MapIoError as _;
	pub use crate::path::{Path, PathBuf, Utf8Path, Utf8PathBuf, YPath, YPathBuf};

	pub(crate) use color_eyre::Report;
	pub(crate) use color_eyre::eyre::{WrapErr as _, bail, eyre};
	pub(crate) use std::sync::Arc;
	pub use tracing::{debug, error, info, trace, warn};
}

pub mod env;
pub mod fs;
pub mod io;
pub mod path;
mod assert {
	#[macro_export]
	macro_rules! eyre_assert {
		($left:expr $(,)?) => {
			if !$left {
				::color_eyre::eyre::bail!(
					"Assertion failed ({})",
					stringify!($left),
				);
			}
		};
		($left:expr, $($arg:tt)+) => {
			if !$left {
				let string = ::std::format!($($arg)+);
				::color_eyre::eyre::bail!(
					"Assertion failed ({}) \n{string}",
					stringify!($left),
				);
			}
		};
	}

	#[macro_export]
	macro_rules! eyre_assert_eq {
		($left:expr, $right:expr $(,)?) => {
			if $left != $right {
				::color_eyre::eyre::bail!(
					"Assertion failed: {:?} ({}) != {:?} ({})",
					$left,
					stringify!($left),
					$right,
					stringify!($right)
				);
			}
		};
		($left:expr, $right:expr, $($arg:tt)+) => {
			if $left != $right {
				let string = ::std::format!($($arg)+);
				::color_eyre::eyre::bail!(
					"Assertion failed: {:?} ({}) != {:?} ({})\n{string}",
					$left,
					stringify!($left),
					$right,
					stringify!($right)
				);
			}
		};
	}

	pub use eyre_assert;
	pub use eyre_assert_eq;
}
