pub mod prelude {
	pub(crate) use camino::Utf8PathBuf;
	pub(crate) use rand::Rng;
	pub(crate) use std::future::{Future, IntoFuture};
	pub(crate) use surrealdb::engine::any::Any;
	pub(crate) use surrealdb::Surreal;
	pub(crate) use tracing::*;
}

pub mod config {
	//! All use cases for information regarding connecting to sureal db databases:
	//! - Testing against an in-memory database, can use fake testing credentials
	//! - Human configuring/testing against production database (can be feature flagged out with "production")
	//! - Shipped configurations for clients to connect WITHOUT ROOT CREDENTIALS to a production database
	//!
	//! Ideal use case: `ymap` crate defines its own ProductionConfig that loads secrets

	use surrealdb::opt::auth::{Jwt, Root};

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

	/// Provides credentials to sign into the root of a database.
	///
	/// Useful for importing into testing/production databases.
	pub trait DBRootCredentials: Send + Sync {
		fn root_username(&self) -> String {
			"root".into()
		}

		fn root_password(&self) -> String;

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
	}

	/// All config to start a new database instance,
	/// for testing or for production.
	pub trait StartDBConfig: Send + Sync {
		/// whether to pass the --strict flag to surreal --start
		fn strict(&self) -> bool {
			true
		}

		/// whether to pass the --auth flag to surreal --start
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

		/// Arguments to pass to `surreal start`, e.g. `--password`.
		///
		/// Only used for production databases.
		fn get_cli_args(&self) -> Vec<String>
		where
			Self: DBRootCredentials,
		{
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

		/// *Assumes you have already switch to primary database and namespace.*
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
	}

	/// All information for clients to connect to the **production** database instance.
	///
	/// Should be the only trait that the client config needs to implement.
	pub trait ConnectRemoteDBConfig: Send + Sync {
		/// What namespace to connect to by default
		fn primary_namespace(&self) -> String;

		/// What database to connect to by default
		fn primary_database(&self) -> String;

		/// e.g. cloud.surrealdb.com
		///
		/// Similar to [StartDBConfig::bind_host]
		fn connect_host(&self) -> String;

		/// Usually [StartDBConfig::bind_port]
		fn connect_port(&self) -> u16;

		/// Connects to database without signing in or initializing.
		fn connect_ws(
			&self,
		) -> impl Future<Output = Result<Surreal<Any>, surrealdb::Error>> + Send + Sync {
			let host = self.connect_host();
			let port = self.connect_port();
			surrealdb::engine::any::connect(format!("ws://{host}:{port}")).into_future()
		}
	}

	/// Start a new in-memory database for **testing only**.
	/// Signs in as root, switches to primary database and namespace, and inits as well.
	///
	/// You must unwrap the option first before calling `.await`.
	pub fn start_in_memory<Config>(
		config: &Config,
	) -> Option<impl Future<Output = Result<Surreal<Any>, surrealdb::Error>> + Send + Sync + '_>
	where
		Config: StartDBConfig + DBRootCredentials + ConnectRemoteDBConfig,
	{
		if let StartDBType::Mem = config.db_type() {
			Some(async {
				let db = match surrealdb::engine::any::connect("memory".to_owned()).await {
					Ok(db) => db,
					Err(err) => return Err(err),
				};
				db.use_ns(config.primary_namespace()).use_db(config.primary_database()).await?;
				config.root_sign_in(&db).await?;
				config.root_init(&db).await?;
				Ok(db)
			})
		} else {
			warn!("Called `config.start_in_memory()` but wasn't a memory DB configuration.");
			None
		}
	}
}

pub mod configs {
	use crate::prelude::*;

	/// Constructs an in-memory database for testing purposes.
	pub struct TestingMem {
		pub port: u16,
		pub username: String,
		pub password: String,
	}

	impl TestingMem {
		pub fn new(port: u16) -> Self {
			TestingMem {
				port,
				username: String::from("testing-username"),
				password: String::from("testing-password"),
			}
		}

		/// Generates a [TestingMem] with a random port between 10000 and 20000.
		pub fn rand() -> Self {
			let mut rand = rand::thread_rng();
			TestingMem::new(rand.gen_range(10000..20000))
		}
	}
}
