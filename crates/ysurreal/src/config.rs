//! All use cases for information regarding connecting to sureal db databases:
//! - Testing against an in-memory database, can use fake testing credentials
//! - Human configuring/testing against production database (can be feature flagged out with "production")
//! - Shipped configurations for clients to connect WITHOUT ROOT CREDENTIALS to a production database
//!
//! Ideal use case: `ymap` crate defines its own ProductionControllerConfig that loads secrets

use surrealdb::{
	opt::auth::{Jwt, Root},
	Connection,
};

use crate::prelude::*;

/// Options for DB engine implementation
#[derive(Debug, PartialEq)]
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

	/// Signs in with root credentials.
	/// Also switches the namespace and database to the primary namespace and database,
	/// which is probably what you wanted.
	fn root_sign_in(
		&self,
		db: &Surreal<Any>,
	) -> impl Future<Output = Result<Jwt, surrealdb::Error>> + Send + Sync
	where
		Self: DBConnectRemoteConfig,
	{
		async {
			debug!("Signing into database with root credentials");
			db.use_ns(self.primary_namespace())
				.use_db(self.primary_database())
				.await?;
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
pub trait DBStartConfig: Send + Sync {
	/// whether to pass the --strict flag to surreal --start
	fn strict(&self) -> bool {
		// true
		// TODO set back to true
		false
	}

	/// whether to pass the --auth flag to surreal --start
	fn auth(&self) -> bool {
		true
	}

	/// e.g. 8000
	fn bind_port(&self) -> u16;

	/// E.g. 0.0.0.0
	fn bind_host(&self) -> String {
		"0.0.0.0".into()
	}

	/// usually 0.0.0.0:8000
	///
	/// Provided for you, automatically combines [StartDBConfig::bind_host] and [StartDBConfig::bind_port].
	fn bind_full_host(&self) -> String {
		format!("{}:{}", self.bind_host(), self.bind_port())
	}

	/// Whether its a [DBType::Mem] or [DBType::File]
	fn db_type(&self) -> StartDBType;

	/// Arguments to pass to `surreal start`, e.g. `--password`.
	fn get_cli_args(&self) -> Vec<String>
	where
		Self: DBRootCredentials,
	{
		let raw_password = self.root_password();
		// maybe find a better way than this
		assert!(
			!raw_password.contains('`'),
			"Cannot use backticks ` in password, else can't escape to nushell over the wire"
		);
		let escaped_password = format!("`{}`", raw_password);
		// trace!(?escaped_password, %escaped_password);

		let mut args = vec![
			"--username".into(),
			self.root_username(),
			"--password".into(),
			escaped_password,
			"--bind".into(),
			self.bind_full_host(),
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

	/// Initializes the database using [DBStartConfig::init_surql].
	/// *Assumes you have already switch to primary database and namespace.*
	fn init_query(
		&self,
		db: &Surreal<Any>,
	) -> impl Future<Output = Result<(), surrealdb::Error>> + Send + Sync {
		async {
			let init_surql = self.init_surql();
			debug!(
				message = "Initializing database with SurrealQL",
				length_of_surql = init_surql.len()
			);
			db.query(&init_surql).await?;
			Ok(())
		}
	}
}

/// All information for clients to connect to the **production** database instance.
///
/// Should be the only trait that the client config needs to implement.
pub trait DBConnectRemoteConfig: Send + Sync {
	/// What namespace to connect to by default
	fn primary_namespace(&self) -> String;

	/// What database to connect to by default
	fn primary_database(&self) -> String;

	fn use_primary_ns_db<C: Connection>(
		&self,
		db: &Surreal<C>,
	) -> impl Future<Output = Result<(), surrealdb::Error>> + Send + Sync {
		async {
			db.use_ns(self.primary_namespace())
				.use_db(self.primary_database())
				.await?;
			Ok(())
		}
	}

	/// e.g. cloud.surrealdb.com
	///
	/// Similar to [StartDBConfig::bind_full_host]
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

	/// Returns the arguments to `surreal sql` to successfully connect.
	///
	/// Requires [DBRootCredentials] because it signs in as root to execute its commands.
	/// Requires [ConnectRemoteDBConfig] because it needs the host, port, namespace and database.
	fn get_sql_cli_args(&self) -> Vec<String>
	where
		Self: DBRootCredentials + DBConnectRemoteConfig,
	{
		vec![
			"--pretty".into(),
			"--auth-level".into(),
			"root".into(),
			"--username".into(),
			self.root_username(),
			"--password".into(),
			self.root_password(),
			"--endpoint".into(),
			format!("ws://{}:{}", self.connect_host(), self.connect_port()),
			"--namespace".into(),
			self.primary_namespace(),
			"--database".into(),
			self.primary_database(),
		]
	}
}

impl<'c, C> DBConnectRemoteConfig for &'c C
where
	C: DBConnectRemoteConfig + Send + Sync,
{
	fn primary_namespace(&self) -> String {
		C::primary_namespace(self)
	}

	fn primary_database(&self) -> String {
		C::primary_database(self)
	}

	fn connect_host(&self) -> String {
		C::connect_host(self)
	}

	fn connect_port(&self) -> u16 {
		C::connect_port(self)
	}
}
