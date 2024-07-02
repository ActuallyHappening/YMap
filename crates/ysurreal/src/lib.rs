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
}

pub mod production;

pub mod testing;
