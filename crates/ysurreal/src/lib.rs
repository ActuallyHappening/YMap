pub mod prelude {
	pub(crate) use clap::Args;
	pub(crate) use tracing::*;

	pub use crate::args::ProductionDBConnection;
}

pub mod args {
	use clap::ValueEnum;
	use surrealdb::{
		engine::remote::ws::{Client, Ws},
		Surreal,
	};

	use crate::prelude::*;

	/// Options for connecting to local or remote surrealdb.
	///
	/// Primary usecase is to turn into [surrealdb::Surreal] instance.
	#[derive(Args, Debug, Clone)]
	pub struct ProductionDBConnection {
		/// Must pass this flag to indicate operating on production db.
		#[arg(long)]
		production: bool,

		/// Without protocol specifier, e.g. localhost:8000
		#[arg(long, env = "_SURREAL_HOST_PRODUCTION")]
		connection: String,

		#[arg(long, env = "SURREAL_DATABASE_PRODUCTION")]
		database: String,

		#[arg(long, env = "SURREAL_NAMESPACE_PRODUCTION")]
		namespace: String,
	}

	impl ProductionDBConnection {
		pub async fn connect(&self) -> Result<Surreal<Client>, surrealdb::Error> {
			let db = Surreal::new::<Ws>(&self.connection).await?;
			db.use_ns(&self.namespace).use_db(&self.database).await?;

			db.wait_for(surrealdb::opt::WaitFor::Database).await;

			Ok(db)
		}
	}

	#[derive(Args, Debug, Clone)]
	pub struct TestingDBConnection {
		/// Without protocol specifier, e.g. localhost:8000
		#[arg(long, env = "_SURREAL_HOST_TEST")]
		connection: String,

		#[arg(long, env = "SURREAL_DATABASE_TEST")]
		database: String,

		#[arg(long, env = "SURREAL_NAMESPACE_TEST")]
		namespace: String,
	}

	impl TestingDBConnection {
		pub async fn connect(&self) -> Result<Surreal<Client>, surrealdb::Error> {
			let db = Surreal::new::<Ws>(&self.connection).await?;
			db.use_ns(&self.namespace).use_db(&self.database).await?;

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
