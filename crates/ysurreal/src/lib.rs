pub mod prelude {
	pub(crate) use rand::Rng;
	pub(crate) use std::future::{Future, IntoFuture};
	pub(crate) use surrealdb::engine::any::Any;
	pub(crate) use surrealdb::Surreal;
	pub(crate) use tracing::*;
	pub(crate) use camino::Utf8PathBuf;
}

pub mod config {
	use crate::prelude::*;

	pub enum DBType {
		File { data_path: Utf8PathBuf },
		Mem,
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

		/// Start a new in-memory database.
		///
		/// You must unwrap the option first before calling `.await`.
		fn start_in_memory(
			&self,
		) -> Option<impl Future<Output = Result<Surreal<Any>, surrealdb::Error>> + Send + Sync> {
			if let DBType::Mem = self.db_type() {
				Some(surrealdb::engine::any::connect("memory".to_owned()).into_future())
			} else {
				warn!("Called `config.start_in_memory()` but wasn't a memory DB configuration.");
				None
			}
		}
	}
}

pub mod configs {
	use crate::prelude::*;

	/// Constructs an in-memory database for testing purposes.
	pub struct TestingMem {
		pub port: u16,
	}

	impl TestingMem {
		pub fn new(port: u16) -> Self {
			TestingMem { port }
		}

		/// Generates a [TestingMem] with a random port between 10000 and 20000.
		pub fn rand() -> Self {
			let mut rand = rand::thread_rng();
			TestingMem::new(rand.gen_range(10000..20000))
		}
	}
}

pub mod local {
	use crate::prelude::*;
}
