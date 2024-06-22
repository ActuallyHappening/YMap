use camino::Utf8PathBuf;
use color_eyre::{
	eyre::{eyre, Context, Report},
	Section, SectionExt,
};
use std::{
	env,
	ffi::OsStr,
	path::PathBuf,
};
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

fn main() -> Result<(), color_eyre::Report> {
	// init error handling and tracing
	{
		install_tracing();
		color_eyre::install().expect("Error reporting couldn't be installed (lol)");
	}

	let mut release_profile = false;
	if env::var_os("CONFIGURATION") == Some(String::from("Debug").into()) {
		release_profile = true;
	}

	// Add `$HOME/.cargo/bin` to PATH env variable
	// Adds /opt/homebrew/bin to PATH env variable
	{
		if let Some(path) = env::var_os("PATH") {
			let mut paths = env::split_paths(&path).collect::<Vec<_>>();

			let home_dir = &dirs::home_dir().unwrap_or("~".into());
			paths.push(home_dir.clone());

			let homebrew_dir = PathBuf::from("/opt/homebrew/bin");
			paths.push(homebrew_dir.clone());

			let new_path = env::join_paths(paths.clone())
				.wrap_err("Unable to add path to PATH env variable")
				.note(format!("Original PATH: {:?}", paths.clone()))
				.note(format!("Home path: {:?}", home_dir))
				.note(format!("Homebrew path: {:?}", homebrew_dir))?;
			env::set_var("PATH", new_path.clone());

			info!(message = "Updated PATH env variable", from = ?path, to = ?new_path, ?home_dir, ?homebrew_dir);
		} else {
			info!("No PATH env variable to update");
		}
	}

	{
		// add developer SDK <comment copied from bevy example script>
		// Assume we're in Xcode, which means we're probably cross-compiling.
		// In this case, we need to add an extra library search path for build scripts and proc-macros,
		// which run on the host instead of the target.
		// (macOS Big Sur does not have linkable libraries in /usr/lib/.)
		if let Some(developer_sdk_dir) = env::var_os("DEVELOPER_SDK_DIR") {
			let developer_sdk_dir = Utf8PathBuf::try_from(PathBuf::from(developer_sdk_dir))
				.wrap_err("DEVELOPER_SDK_DIR not UTF8")?;
			// check if directory exits
			let exists = developer_sdk_dir.try_exists();
			match exists {
				Ok(true) => {
					let library_path = env::var_os("LIBRARY_PATH").unwrap_or_default();
					let mut library_paths = env::split_paths(&library_path).collect::<Vec<_>>();
					library_paths.push(PathBuf::from(format!(
						"{}/MacOSZ.sdk/usr/lib",
						developer_sdk_dir
					)));
					let new_library_path =
						env::join_paths(library_paths).wrap_err("Cannot join paths for DEVELOPER_SDK_DIR")?;

					env::set_var("LIBRARY_PATH", &new_library_path);

					info!(message = "Updated LIBRARY_PATH env variable with a subpath of DEVELOPER_SDK_DIR env variable", ?developer_sdk_dir, from = ?library_path, to = ?new_library_path);
				}
				_ => {
					return Err(
						eyre!("DEVELOPER_SDK_DIR does not exist or not UTF8")
							.with_section(|| developer_sdk_dir.header("DEVELOPER_SDK_DIR:")),
					)
				}
			}
		}
	}

	let mut is_simulator = false;
	if let Some(llvm_target_triple_suffix) = env::var_os("LLVM_TARGET_TRIPLE_SUFFIX") {
		if llvm_target_triple_suffix == *"-simulator" {
			info!(
				message = "Assuming building for a simulator",
				?llvm_target_triple_suffix
			);
			is_simulator = true;
		}
	}

	match parse_archs()? {
		Archs::arm64 => {
			if is_simulator {
				return Err(eyre!(
					"Building for x86_64 but not on a simulator. This is not yet supported"
				));
			}

			// Intel iOS simulator
			env::set_var("CFLAGS_x86_64_apple_ios", "-targetx86_64-apple-ios");

			let mut rustc = bossy::Command::pure("cargo").with_args([
				"rustc",
				"--crate-type",
				"staticlib",
				"--lib",
				"--target",
				"x86_64-apple-ios",
			]);
			if release_profile {
				rustc.add_arg("--release");
			}
			rustc.run_and_wait() ;
		}
	}

	Ok(())
}

enum Archs {
	x86_64,
	arm64,
}

fn parse_archs() -> color_eyre::Result<Archs> {
	impl Archs {
		fn try_from_str(str: &str) -> Result<Self, Report> {
			Ok(match str {
				"x86_64" => Archs::x86_64,
				"arm64" => Archs::arm64,
				_ => {
					return Err(eyre!("Cannot parse ARCHS env variable").note(format!("ARCHS: {:?}", str)))
				}
			})
		}

		fn try_from_os_str(str: &OsStr) -> Result<Self, Report> {
			match str.to_str() {
				Some(str) => Self::try_from_str(str),
				None => Err(eyre!("ARCHS env variable is not UTF8")),
			}
		}
	}

	match env::var_os("ARCHS") {
		None => Err(eyre!("No ARCHS env var present")),
		Some(archs) => Archs::try_from_os_str(&archs),
	}
}

fn install_tracing() {
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
