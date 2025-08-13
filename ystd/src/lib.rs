pub mod prelude;

#[cfg(feature = "base64")]
pub mod base64;
pub mod cli;
pub mod env;
pub mod error;
pub mod fs;
#[cfg(feature = "hex")]
pub mod hex;
pub mod io;
pub mod path;
pub mod string;
pub mod sync;
pub mod time;
pub use url;
#[cfg(feature = "which")]
pub mod which;
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
