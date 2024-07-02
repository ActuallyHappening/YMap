#![cfg_attr(test, allow(unused_imports))]

pub mod prelude {
	#![allow(unused_imports)]

	pub(crate) use camino::Utf8PathBuf;
	pub(crate) use clap::Args;
	pub(crate) use clap::Subcommand;
	pub(crate) use color_eyre::eyre::WrapErr;
	pub(crate) use tracing::*;
	pub(crate) use tracing::*;
	pub(crate) use which::which;

	pub(crate) use crate::impl_from_env;
}

pub mod production;

pub mod testing;

pub trait FromEnv {
	fn try_from_env() -> Result<Self, color_eyre::Report>
	where
		Self: Sized;

	/// Constructs a new instance from the environment variables only.
	///
	/// Usually requires that the `env.nu` file has already been `source`d.
	fn from_env() -> Self
	where
		Self: Sized,
	{
		match Self::try_from_env() {
			Ok(val) => val,
			Err(err) => {
				eprintln!(
					"Couldn't fully retrieve {} from env: {}",
					std::any::type_name::<Self>(),
					err
				);
				std::process::exit(1);
			}
		}
	}
}

#[macro_export]
macro_rules! impl_from_env {
	($ty:ty) => {
		impl $crate::FromEnv for $ty {
			fn try_from_env() -> Result<Self, color_eyre::Report> {
				use clap::Parser;

				#[derive(Parser)]
				struct ParseMe {
					#[clap(flatten)]
					data: $ty,
				}

				let data = ParseMe::try_parse_from([&""]).wrap_err(
					"Couldn't parse from env variables, have you imported the `env.nu` file before running?",
				)?;
				Ok(data.data)
			}
		}
	};
}
