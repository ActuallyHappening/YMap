pub mod prelude {
	pub(crate) use camino::Utf8PathBuf;
	pub(crate) use rand::Rng;
	pub(crate) use std::future::{Future, IntoFuture};
	pub(crate) use surrealdb::engine::any::Any;
	pub(crate) use surrealdb::Surreal;
	pub(crate) use tracing::*;
}

pub mod config {
	use surrealdb::{
		engine::remote::ws::Ws,
		opt::auth::{Jwt, Root},
	};

	use crate::prelude::*;

	/// Options for DB engine implementation
	pub enum StartDBType {
		File { data_path: Utf8PathBuf },
		Mem,
	}

	impl StartDBType {
		/// `file://foo.db` or `memory`
		pub fn get_start_address(&self) -> String {
			match self {
				StartDBType::File { data_path } => format!("file://{}", data_path),
				StartDBType::Mem => "memory".into(),
			}
		}
	}

	/// For database configurations that include a root username and password.
	/// 
	/// Configurations shipped to clients should not include these
	pub trait RootDBConfig: Send + Sync {

	}

	/// All config to start, connect and root manage a new database instance,
	/// for testing or for production.
	pub trait StartDBConfig: Send + Sync {
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

		/// e.g. 8000
		fn bind_port(&self) -> u16;

		/// usually 0.0.0.0:8000
		fn bind_host(&self) -> String {
			format!("0.0.0.0:{}", self.bind_port())
		}

		/// Whether its a [DBType::Mem] or [DBType::File]
		fn db_type(&self) -> StartDBType;

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
			// this goes last
			args.push(self.db_type().get_start_address());
			args
		}

		/// Returns the SurealQL queries to initialize the database.
		fn init_surql(&self) -> String;

		/// e.g. cloud.surrealdb.com
		fn connect_host(&self) -> String;

		/// Usually [StartDBConfig::bind_port]
		fn connect_port(&self) -> u16;

		fn root_sign_in(
			&self,
			db: &Surreal<Any>,
		) -> impl Future<Output = Result<Jwt, surrealdb::Error>> + Send + Sync {
			async {
				debug!("Signing into database with root credentials");
				db.signin(Root {
					username: self.root_username().as_str(),
					password: self.root_password().as_str(),
				})
				.await
			}
		}

		fn root_init(
			&self,
			db: &Surreal<Any>,
		) -> impl Future<Output = Result<(), surrealdb::Error>> + Send + Sync {
			async {
				debug!("Initializing database with SurrealQL");
				db.query(self.init_surql().as_str()).await?;
				Ok(())
			}
		}

		/// Connects to database without signing in or initializing.
		fn connect_ws(
			&self,
		) -> impl Future<Output = Result<Surreal<Any>, surrealdb::Error>> + Send + Sync {
			let host = self.connect_host();
			let port = self.connect_port();
			surrealdb::engine::any::connect(format!("ws://{host}:{port}")).into_future()
		}

		/// Start a new in-memory database. Signs in and inits as well.
		///
		/// You must unwrap the option first before calling `.await`.
		fn start_in_memory(
			&self,
		) -> Option<impl Future<Output = Result<Surreal<Any>, surrealdb::Error>> + Send + Sync> {
			if let StartDBType::Mem = self.db_type() {
				Some(async {
					let db = match surrealdb::engine::any::connect("memory".to_owned()).await {
						Ok(db) => db,
						Err(err) => return Err(err),
					};
					self.root_sign_in(&db).await?;
					self.root_init(&db).await?;
					Ok(db)
				})
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
