pub mod prelude {
	pub(crate) use tracing::*;
}

pub mod config {
	use camino::Utf8PathBuf;

use crate::prelude::*;

	pub enum DBType {
		File {
			data_path: Utf8PathBuf,
		},
		Mem
	}

	/// All config to start a new database instance,
	/// for testing or for production.
	pub trait StartDBConfig {
		fn root_username(&self) -> String {
			"root".into()
		}

		fn root_password(&self) -> String;

		fn primary_namespace(&self) -> String;

		fn primary_database(&self) -> String;

		fn strict(&self) -> bool {
			true
		}

		fn auth(&self) -> bool {
			true
		}

		fn bind_port(&self) -> u16;

		fn bind_host(&self) -> String {
			format!("0.0.0.0:{}", self.bind_port())
		}

		/// Whether its a [DBType::Mem] or [DBType::File]
		fn db_type(&self) -> DBType;

		/// Arguments to pass to `surreal start`, e.g. `--password`
		fn get_cli_args(&self) -> Vec<String> {
			let mut args = vec![
				"--username".into(),
				self.root_username(),
				"--password".into(),
				self.root_password(),
				"--bind".into(),
				self.bind_host(),
			];
			if self.auth() {
				args.push("--auth".into());
			}
			if self.strict() {
				args.push("--strict".into())
			}
			match self.db_type() {
				DBType::File { data_path } => {
					args.push(format!("file://{}", data_path));
				}
				DBType::Mem => {
					args.push("memory".into());
				}
			}
			args
		}
	}
}

pub mod local {
	use crate::prelude::*;
}
