use crate::prelude::*;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Config {
	/// What features to enable for iOS builds
	#[serde(default)]
	ios: Flags,
}

pub use flags::*;
mod flags {
	use crate::prelude::*;

	#[derive(Debug, Deserialize, Clone)]
	pub struct Flags {
		#[serde(default)]
		features: Vec<String>,
		/// Whether or not to pass the flag `--no-default-features` to `cargo rustc`
		/// See https://doc.rust-lang.org/cargo/reference/features.html#command-line-feature-options
		#[serde(
			default = "Flags::default_default_features",
			rename = "default-features"
		)]
		default_features: bool,
		/// Passed to `cargo rustc`
		/// E.g.
		/// ```toml
		/// extra_flags = ["--cfg", "winit_ignore_noise_logs_unstable"]
		/// ```
		#[serde(default, rename = "extra-flags")]
		extra_flags: Vec<String>,
	}

	impl Flags {
		/// Default for [Self::default_features]
		fn default_default_features() -> bool {
			true
		}
	}

	impl Flags {
		pub fn into_args(&self) -> Vec<String> {
			let mut args = vec![];
			if !self.default_features {
				args.push("--no-default-features".into());
			}
			for feature in self.features.iter() {
				args.push("--features".into());
				args.push(feature.clone());
			}
			for extra_flags in self.extra_flags.iter() {
				args.push(extra_flags.clone());
			}
			args
		}
	}

	impl Default for Flags {
		fn default() -> Self {
			Flags {
				default_features: Flags::default_default_features(),
				features: Default::default(),
				extra_flags: Default::default(),
			}
		}
	}

	// #[cfg(test)]
	// mod tests {
	// 	use super::*;

	// 	fn test_basic_toml() {
	// 		let raw_toml: r##"
	// 		[package.metadata.xcode-build-rs.ios]

	// 		"##
	// 	}
	// }
}

impl Config {
	pub fn retrieve_from_toml_config(manifest_path: &Utf8Path) -> Result<Config, Report> {
		match std::fs::read_to_string(manifest_path) {
			Err(err) => {
				info!(
					message = "Cannot find `Cargo.toml` file in manifest_dir, using default config",
					?err,
					?manifest_path
				);
				Ok(Config::default())
			}
			Ok(config) => {
				let raw_config: toml::Value = toml::from_str(&config)
					.wrap_err_with(|| format!("Cannot parse Cargo.toml file: {:?}", manifest_path))?;
				let config = raw_config
					.get("package")
					.and_then(|package| package.get("metadata"))
					.and_then(|metadata| metadata.get("xcode-build-rs"));
				match config {
					None => {
						info!(?manifest_path, "Using default config since `package.metadata.xcode_build_rs` section is missing from Cargo.toml");
						Ok(Config::default())
					}
					Some(toml_config) => {
						let config: Config = toml_config
							.clone()
							.try_into()
							.wrap_err("Cannot deserialize `xcode-build-rs` section of Cargo.toml")?;
						info!(message = "Deserialized Config from Cargo.toml", ?config, ?manifest_path, cwd = ?cwd()?, ?toml_config);
						Ok(config)
					}
				}
			}
		}
	}

	pub fn ios_feature_flags(&self) -> Flags {
		self.ios.clone()
	}
}
