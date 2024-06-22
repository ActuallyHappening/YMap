use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};
use color_eyre::{
	eyre::{eyre, Context, ContextCompat, Report},
	Section, SectionExt,
};
use std::env;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

use xcode_build_rs::*;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
	#[clap(subcommand)]
	pub mode: Mode,

	#[clap(flatten)]
	pub options: Options,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Mode {
	Xcode,
	Test,
}

#[derive(Args, Debug)]
pub struct Options {
	/// By default, doesn't display colour because this can be annoying in the XCode terminal
	#[arg(long, alias = "colour")]
	pub colour: bool,
}

fn main() -> Result<(), color_eyre::Report> {
	let args = Cli::parse();

	// init error handling and tracing
	{
		install_tracing(args.options.colour);
		color_eyre::install().expect("Error reporting couldn't be installed (lol)");
	}

	run_script(args)
}

fn run_script(args: Cli) -> Result<(), Report> {
	// log all environment variables
	{
		let vars = env::vars().collect::<std::collections::HashMap<_, _>>();
		info!(?vars);
	}

	// release profile
	let is_release_build = release_profile()?;

	append_useful_paths()?;

	append_library_paths()?;

	let is_simulator = is_simulator()?;

	if let Mode::Test = args.mode {
		info!("Skipping actual compilation, running a test rustc build for M1 iOS simulator");

		rustc("aarch64-apple-ios-sim", is_release_build)?;

		return Ok(());
	}

	match parse_archs()? {
		Archs::X86_64 => {
			if is_simulator {
				return Err(eyre!(
					"Building for x86_64 but not on a simulator. This is not yet supported"
				));
			}

			// Intel iOS simulator
			// this talks to the `cc` compiler rust library
			// https://docs.rs/cc/latest/cc/#external-configuration-via-environment-variables
			// these end up being passed to the underlying C compiler
			env::set_var("CFLAGS_x86_64_apple_ios", "-targetx86_64-apple-ios");

			rustc("x86_64-apple-ios", is_release_build)?;
		}
		Archs::Arm64 => {
			if is_simulator {
				// M1 iOS simulator
				rustc("aarch64-apple-ios-sim", is_release_build)?;
			} else {
				// Hardware iOS
				rustc("aarch64-apple-ios", is_release_build)?;
			}
		}
	}

	Ok(())
}

pub fn install_tracing(ansi: bool) {
	use tracing_error::ErrorLayer;
	use tracing_subscriber::prelude::*;
	use tracing_subscriber::{fmt, EnvFilter};

	let mut fmt_layer = fmt::Layer::default().with_target(false);
	fmt_layer.set_ansi(ansi);

	let filter_layer = EnvFilter::try_from_default_env()
		.or_else(|_| EnvFilter::try_new("info"))
		.unwrap();

	tracing_subscriber::registry()
		.with(filter_layer)
		.with(fmt_layer)
		.with(ErrorLayer::default())
		.init();
}

/// Add `$HOME/.cargo/bin` to PATH env variable
/// Adds /opt/homebrew/bin to PATH env variable
fn append_useful_paths() -> Result<(), Report> {
	if let Ok(path) = env::var("PATH") {
		let mut paths = env::split_paths(&path).collect::<Vec<_>>();

		let home_dir = dirs::home_dir().wrap_err("Could not find home dir")?;
		let home_dir = Utf8PathBuf::try_from(home_dir).wrap_err("Home dir not UTF8")?;
		paths.push(home_dir.clone().into());

		let cargo_bin_dir = home_dir.join(".cargo").join("bin");
		paths.push(cargo_bin_dir.clone().into());

		let homebrew_dir = Utf8PathBuf::from("/opt/homebrew/bin");
		paths.push(homebrew_dir.clone().into());

		// debug check for `cc`
		if !debug_confirm_on_path(&paths, "cc")? {
			info!("`cc` compiler not found on $PATH");
		}

		let new_path = env::join_paths(paths.clone())
			.wrap_err("Unable to add path to PATH env variable")
			.note(format!("Original PATH: {:?}", paths.clone()))
			.note(format!("+Cargo bin dir: {:?}", cargo_bin_dir))
			.note(format!("+Home path: {:?}", home_dir))
			.note(format!("+Homebrew path: {:?}", homebrew_dir))?;
		env::set_var("PATH", new_path.clone());

		info!(
			message = "Updated PATH env variable",
			note = "To add cargo_bin_dir, home_dir and homebrew_dir",
			?cargo_bin_dir,
			?home_dir,
			?homebrew_dir,
		);
		debug!(message = "Path env variable updated", from = ?path, to = ?new_path);
	} else {
		info!("No PATH env variable to update");
	}

	Ok(())
}

/// add developer SDK <comment copied from bevy example script>
/// Assume we're in Xcode, which means we're probably cross-compiling.
/// In this case, we need to add an extra library search path for build scripts and proc-macros,
/// which run on the host instead of the target.
/// (macOS Big Sur does not have linkable libraries in /usr/lib/.)
fn append_library_paths() -> Result<(), Report> {
	if let Ok(developer_sdk_dir) = env::var("DEVELOPER_SDK_DIR") {
		let developer_sdk_dir = Utf8PathBuf::from(developer_sdk_dir);
		// check if directory exits
		let exists = developer_sdk_dir.try_exists();
		match exists {
				Ok(true) => {
					let library_path = env::var("LIBRARY_PATH").unwrap_or_default();
					let mut library_paths = env::split_paths(&library_path).collect::<Vec<_>>();
					library_paths.push(Utf8PathBuf::from(format!(
						"{}/MacOSX.sdk/usr/lib",
						developer_sdk_dir
					)).into());
					let new_library_path =
						env::join_paths(library_paths).wrap_err("Cannot join paths for DEVELOPER_SDK_DIR")?;

					env::set_var("LIBRARY_PATH", &new_library_path);

					info!(message = "Updated LIBRARY_PATH env variable with a subpath of DEVELOPER_SDK_DIR env variable", ?developer_sdk_dir, from = ?library_path, to = ?new_library_path);
				}
				_ => {
					return Err(
						eyre!("DEVELOPER_SDK_DIR does not exist or not UTF8")
        .note("The env variable DEVELOPER_SDK_DIR was found, but the path it pointed to doesn't exist")
							.with_section(|| developer_sdk_dir.header("DEVELOPER_SDK_DIR:")),
					)
				}
			}
	} else {
		info!(
			message = "No DEVELOPER_SDK_DIR env variable found in environment",
			note = "Not updating LIBRARY_PATH env variable"
		);
	}

	Ok(())
}
