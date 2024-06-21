use camino::Utf8PathBuf;
use color_eyre::{
	eyre::{eyre, Context},
	Section, SectionExt,
};
use std::{
	env,
	path::{Path, PathBuf},
};
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

fn main() -> Result<(), color_eyre::Report> {
	// init error handling and tracing
	{
		install_tracing();
		color_eyre::install().expect("Error reporting couldn't be installed (lol)");
	}

	// Add `$HOME/.cargo/bin` to PATH env variable
	{
		let home = &dirs::home_dir().unwrap_or("~".into());
		if let Some(path) = env::var_os("PATH") {
			let mut paths = env::split_paths(&path).collect::<Vec<_>>();
			paths.push(home.clone());
			let new_path = env::join_paths(paths.clone())
				.wrap_err("Unable to add path to PATH env variable")
				.note(format!("Home path: {:?}", home))
				.note(format!("Original PATH: {:?}", paths.clone()))?;
			env::set_var("PATH", new_path.clone());

			info!(message = "Updated PATH env variable", from = ?path, to = ?new_path);
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

					env::set_var("LIBRARY_PATH", new_library_path);
				}
				_ => {
					return Err(
						eyre!("DEVELOPER_SDK_DIR does not exist or not UTF8").with_section(|| {
							developer_sdk_dir
								.header("DEVELOPER_SDK_DIR:")
						}),
					)
				}
			}
		}
	}

	Ok(())
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
