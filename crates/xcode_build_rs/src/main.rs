use camino::Utf8PathBuf;
use color_eyre::{
	eyre::{eyre, Context},
	Section, SectionExt,
};
use std::env;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

use xcode_build_rs::*;

fn main() -> Result<(), color_eyre::Report> {
	// init error handling and tracing
	{
		install_tracing();
		color_eyre::install().expect("Error reporting couldn't be installed (lol)");
	}

	let mut release_profile = false;
	if env::var("CONFIGURATION") == Ok("Debug".into()) {
		release_profile = true;
	}

	// Add `$HOME/.cargo/bin` to PATH env variable
	// Adds /opt/homebrew/bin to PATH env variable
	{
		if let Ok(path) = env::var("PATH") {
			let mut paths = env::split_paths(&path).collect::<Vec<_>>();

			let home_dir = &dirs::home_dir().unwrap_or("~".into());
			paths.push(home_dir.clone());

			let homebrew_dir = Utf8PathBuf::from("/opt/homebrew/bin");
			paths.push(homebrew_dir.clone().into());

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
		if let Ok(developer_sdk_dir) = env::var("DEVELOPER_SDK_DIR") {
			let developer_sdk_dir = Utf8PathBuf::from(developer_sdk_dir);
			// check if directory exits
			let exists = developer_sdk_dir.try_exists();
			match exists {
				Ok(true) => {
					let library_path = env::var("LIBRARY_PATH").unwrap_or_default();
					let mut library_paths = env::split_paths(&library_path).collect::<Vec<_>>();
					library_paths.push(Utf8PathBuf::from(format!(
						"{}/MacOSZ.sdk/usr/lib",
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
		Archs::X86_64 => {
			if is_simulator {
				return Err(eyre!(
					"Building for x86_64 but not on a simulator. This is not yet supported"
				));
			}

			// Intel iOS simulator
			// tbh I don't know what this does yet, haven't bothered removing it
			env::set_var("CFLAGS_x86_64_apple_ios", "-targetx86_64-apple-ios");

			rustc("x86_64-apple-ios", release_profile)?;
		}
		Archs::Arm64 => {
			if is_simulator {
				// M1 iOS simulator
				rustc("aarch64-apple-ios-sim", release_profile)?;
			} else {
				// Hardware iOS
				rustc("aarch64-apple-ios", release_profile)?;
			}
		}
	}

	Ok(())
}
