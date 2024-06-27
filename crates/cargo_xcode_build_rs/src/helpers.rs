use crate::prelude::*;

pub fn release_profile() -> Result<bool, Report> {
	let mut is_release_build = true;
	const CONFIGURATION: &str = "CONFIGURATION";
	let configuration_env_var = env::var(CONFIGURATION);
	if configuration_env_var == Ok("Release".into()) {
		is_release_build = true;
		info!(
			message =
				"Assuming a --release profile since the CONFIGURATION env flag was set to 'Release'",
			?configuration_env_var
		);
	} else if configuration_env_var == Ok("Debug".into()) {
		is_release_build = false;
		info!(
			message =
				"Assuming not a release profile since the CONFIGURATION env flag was set to 'Debug'",
			?configuration_env_var
		);
	} else {
		info!(
			message = "No known release profile was provided in the CONFIGURATION env var",
			?configuration_env_var,
			?is_release_build
		);
	}
	Ok(is_release_build)
}

pub fn is_simulator() -> Result<bool, Report> {
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
	Ok(is_simulator)
}

pub enum Archs {
	X86_64,
	Arm64,
}

pub fn parse_archs() -> color_eyre::Result<Archs> {
	impl Archs {
		fn try_from_str(str: &str) -> Result<Self, Report> {
			Ok(match str {
				"x86_64" => Archs::X86_64,
				"arm64" | "arm64 x86_64" => Archs::Arm64,
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

pub fn rustc(
	target: &'static str,
	release: bool,
	flags: Flags,
	manifest_dir: &Utf8Path,
) -> Result<(), Report> {
	let rustc_path = which::which("cargo").wrap_err("Cannot find cargo executable path")?;
	// don't change this to pure. just don't.
	let mut rustc = bossy::Command::impure(&rustc_path).with_args([
		"rustc",
		"--crate-type",
		"staticlib",
		"--lib",
		"--target",
		target,
		"--manifest-path",
		manifest_dir.as_str(),
	]);
	for flag in flags.into_args() {
		rustc.add_arg(flag);
	}
	if release {
		rustc.add_arg("--release");
	}
	info!(message = "About to run rustc", cwd = ?cwd(), ?rustc_path, ?flags, ?manifest_dir);
	rustc
		.run_and_wait()
		.wrap_err("rustc invocation failed, likely a Rust-side build error")?;
	Ok(())
}

pub fn cwd() -> Result<Utf8PathBuf, Report> {
	Utf8PathBuf::try_from(env::current_dir().wrap_err("Cannot find cwd")?).wrap_err("CWD is not UTF8")
}

pub fn debug_confirm_on_path(
	paths: &[std::path::PathBuf],
	bin: &'static str,
) -> Result<bool, Report> {
	let bin_path = which::which(bin).wrap_err("Couldn't find binary")?;
	let bin_dir = {
		let mut bin_path = bin_path.clone();
		bin_path.pop();
		bin_path
	};
	let bin_contained = paths.contains(&bin_dir);
	info!(
		?bin_contained,
		?bin_dir,
		?bin_path,
		"Debug searching for `{}` bin path, was successful: {}",
		bin,
		bin_contained
	);
	Ok(bin_contained)
}
