use crate::prelude::*;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(name = "cargo", bin_name = "cargo")]
pub enum TopLevel {
	#[clap(name = "xcode-build-rs")]
	XCodeBuild(XcodeBuild),
}

impl TopLevel {
	fn inner(&self) -> &XcodeBuild {
		match self {
			TopLevel::XCodeBuild(inner) => inner,
		}
	}
}

#[derive(clap::Args, Debug)]
#[command(version, about)]
pub struct XcodeBuild {
	#[clap(subcommand)]
	mode: Mode,

	#[clap(flatten)]
	options: Options,
}

impl TopLevel {
	pub fn options(&self) -> &Options {
		&self.inner().options
	}

	pub fn mode(&self) -> &Mode {
		&self.inner().mode
	}
}

#[derive(Subcommand, Clone, Debug)]
pub enum Mode {
	/// Run in XCode
	Xcode,
	/// Run a test build for an iOS simulator
	Test,
}

#[derive(Args, Debug)]
pub struct Options {
	/// By default, doesn't display colour because this can be annoying in the XCode terminal
	#[arg(long, alias = "colour")]
	pub colour: bool,

	/// The --manifest-path option to pass to `cargo rustc builds`.
	/// Often you can pass `.`
	#[arg(long, alias = "manifest-dir")]
	manifest_dir: Utf8PathBuf,
}

impl Options {
	/// Specifically to the *file* `Cargo.toml`, *not directory*
	pub fn manifest_path(&self) -> Utf8PathBuf {
		self.manifest_dir.to_owned().join("Cargo.toml")
	}
}
