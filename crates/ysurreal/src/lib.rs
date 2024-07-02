pub mod prelude {
	#![allow(unused_imports)]
	
	pub(crate) use camino::Utf8PathBuf;
	pub(crate) use clap::Args;
	pub(crate) use clap::Subcommand;
	pub(crate) use color_eyre::eyre::WrapErr;
	pub(crate) use tracing::*;
	pub(crate) use tracing::*;
	pub(crate) use which::which;

	pub use crate::args::ProductionDBConnection;
}

pub mod production;

pub mod testing;

pub mod args {
	use surrealdb::{
		engine::remote::{
			http::{self, Http},
			ws::{self, Ws},
		},
		opt::auth::Root,
		Surreal,
	};

	use crate::prelude::*;

	/// Options for connecting to the server DB with root credentials.
	///
	/// Primary usecase is to turn into [surrealdb::Surreal] instance.
	#[derive(Args, Debug, Clone)]
	pub struct ProductionDBConnection {
		#[arg(long, env = "SURREAL_USER")]
		pub username: String,

		#[arg(long, env = "SURREAL_PASS")]
		pub password: String,

		/// Without protocol specifier, e.g. localhost:8000
		#[arg(long, env = "_SURREAL_HOST_PRODUCTION")]
		pub address: String,

		#[arg(long, env = "_SURREAL_DATABASE_PRODUCTION")]
		pub database: String,

		#[arg(long, env = "_SURREAL_NAMESPACE_PRODUCTION")]
		pub namespace: String,
	}

	impl ProductionDBConnection {
		pub async fn connect_http(&self) -> Result<Surreal<http::Client>, surrealdb::Error> {
			let address = self.address.as_str();
			let namespace = self.namespace.as_str();
			let database = self.database.as_str();
			let username = self.username.as_str();
			let password = self.password.as_str();
			info!(
				message = "Connecting to production DB",
				?address,
				?namespace,
				?database,
				note = "Waiting for database connection before proceeding"
			);

			let db = Surreal::new::<Http>(address).await?;
			db.use_ns(namespace).use_db(database).await?;
			db.signin(Root { username, password }).await?;
			db.wait_for(surrealdb::opt::WaitFor::Database).await;

			Ok(db)
		}

		pub async fn connect_ws(&self) -> Result<Surreal<ws::Client>, surrealdb::Error> {
			let address = self.address.as_str();
			let namespace = self.namespace.as_str();
			let database = self.database.as_str();
			let username = self.username.as_str();
			let password = self.password.as_str();
			info!(
				message = "Connecting to production DB",
				?address,
				?namespace,
				?database,
				note = "Waiting for database connection before proceeding"
			);

			let db = Surreal::new::<Ws>(address).await?;
			db.use_ns(namespace).use_db(database).await?;
			db.signin(Root { username, password }).await?;
			db.wait_for(surrealdb::opt::WaitFor::Database).await;

			Ok(db)
		}
	}

	/// Options for connecting to the server DB.
	///
	/// Does *not automatically sign into anything*. See [yauth] for custom signin.
	/// Primary usecase is to turn into [surrealdb::Surreal] instance.
	///
	/// Also has root credentials, but DOESN'T automatically sign into as root.
	///
	/// See also [ProductionDBConnection] for root signin.
	#[derive(Args, Debug, Clone)]
	pub struct TestingDBConnection {
		#[arg(long, env = "_SURREAL_USER_TESTING")]
		pub username: String,

		#[arg(long, env = "_SURREAL_PASS_TESTING")]
		pub password: String,

		#[arg(long, env = "_SURREAL_PORT_TESTING")]
		pub port: String,

		/// Without protocol specifier, e.g. localhost:8000
		#[arg(long, env = "_SURREAL_HOST_TESTING")]
		pub address: String,

		#[arg(long, env = "_SURREAL_DATABASE_TESTING")]
		pub database: String,

		#[arg(long, env = "_SURREAL_NAMESPACE_TESTING")]
		pub namespace: String,
	}

	impl TestingDBConnection {
		pub async fn connect_http(&self) -> Result<Surreal<http::Client>, surrealdb::Error> {
			let address = self.address.as_str();
			let namespace = self.namespace.as_str();
			let database = self.database.as_str();
			// let username = self.username.as_str();
			// let password = self.password.as_str();
			info!(
				message = "Connecting to testing DB",
				?address,
				?namespace,
				?database,
				note = "Waiting for database connection before proceeding"
			);

			let db = Surreal::new::<Http>(address).await?;
			db.use_ns(namespace).use_db(database).await?;
			db.wait_for(surrealdb::opt::WaitFor::Database).await;

			Ok(db)
		}

		pub async fn connect_ws(&self) -> Result<Surreal<ws::Client>, surrealdb::Error> {
			let address = self.address.as_str();
			let namespace = self.namespace.as_str();
			let database = self.database.as_str();
			// let username = self.username.as_str();
			// let password = self.password.as_str();
			info!(
				message = "Connecting to testing DB",
				?address,
				?namespace,
				?database,
				note = "Waiting for database connection before proceeding"
			);

			let db = Surreal::new::<Ws>(address).await?;
			db.use_ns(namespace).use_db(database).await?;
			db.wait_for(surrealdb::opt::WaitFor::Database).await;

			Ok(db)
		}
	}
}

pub mod server {
	use openssh::Session;

	use crate::prelude::*;

	#[derive(Args, Debug, Clone)]
	pub struct SSHServerConnection {
		/// What you would type in `ssh <NAME>`.
		/// e.g. ah@example.com, localhost
		///
		/// Does not include port, see [ServerConnectionOptions::ssh_port]
		#[arg(long, env = "YSURREAL_SSH_NAME")]
		pub ssh_name: String,
		// #[arg(long, env = "YSURREAL_SSH_PORT")]
		// pub ssh_port: String,
	}

	impl SSHServerConnection {
		pub async fn connect(&self) -> Result<Session, openssh::Error> {
			let ssh_name = self.ssh_name.as_str();
			info!(message = "Connecting to server host", ?ssh_name);
			Session::connect_mux(ssh_name, openssh::KnownHosts::Strict).await
		}
	}
}
