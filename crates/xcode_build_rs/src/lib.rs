use std::env;

use color_eyre::{
	eyre::{eyre, Context as _, Report},
	Section as _,
};

pub enum Archs {
	X86_64,
	Arm64,
}

pub fn parse_archs() -> color_eyre::Result<Archs> {
	impl Archs {
		fn try_from_str(str: &str) -> Result<Self, Report> {
			Ok(match str {
				"x86_64" => Archs::X86_64,
				"arm64" => Archs::Arm64,
				_ => {
					return Err(eyre!("Cannot parse ARCHS env variable").note(format!("ARCHS: {:?}", str)))
				}
			})
		}
	}

	match env::var("ARCHS") {
		Err(e) => Err(e).wrap_err("No ARCHS env var present")?,
		Ok(archs) => Archs::try_from_str(&archs),
	}
}

pub fn rustc(target: &'static str, release: bool) -> Result<(), Report> {
	let mut rustc = bossy::Command::pure("cargo").with_args([
		"rustc",
		"--crate-type",
		"staticlib",
		"--lib",
		"--target",
		target,
	]);
	if release {
		rustc.add_arg("--release");
	}
	rustc
		.run_and_wait()
		.wrap_err("rustc invocation failed, likely a Rust-side build error")?;
	Ok(())
}

pub fn install_tracing() {
	use tracing_error::ErrorLayer;
	use tracing_subscriber::prelude::*;
	use tracing_subscriber::{fmt, EnvFilter};

	let fmt_layer = fmt::layer().with_target(false);
	let filter_layer = EnvFilter::try_from_default_env()
		.or_else(|_| EnvFilter::try_new("info"))
		.unwrap();

	tracing_subscriber::registry()
		.with(filter_layer)
		.with(fmt_layer)
		.with(ErrorLayer::default())
		.init();
}
