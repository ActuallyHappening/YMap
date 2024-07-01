pub mod prelude {
	pub(crate) use clap::Args;
	pub(crate) use tracing::*;

	pub use crate::args::SurrealConnectionOptions;
}

pub mod args {
	use clap::ValueEnum;
	use surrealdb::{engine::remote::ws::Ws, Surreal};

	use crate::prelude::*;

	#[derive(Args, Debug, Clone)]
	pub struct SurrealConnectionOptions {
		/// Without protocol specifier, e.g. localhost:8000
		#[arg(long, env = "_SURREAL_CONN")]
		pub connection: String,

		#[arg(long, env = "SURREAL_DATABASE")]
		pub database: String,

		#[arg(long, env = "SURREAL_NAMESPACE")]
		pub namespace: String,

		/// When supplied, skips the waiting required to setup the connection
		#[arg(long)]
		pub no_wait: bool,
	}

	// 	impl SurrealConnectionOptions {
	// 		pub async fn connect(&self) -> Result<Surreal<Ws>, surrealdb::Error> {
	// 			let db = Surreal::new(&self.connection).await?;
	// 			db.use_ns(&self.namespace).use_db(&self.database).await?;

	// 			Ok(db)
	// 		}
	// 	}
}

pub mod server {
	use openssh::Session;

	use crate::prelude::*;

	#[derive(Args, Debug, Clone)]
	pub struct ServerConnectionOptions {
		/// What you would type in `ssh <NAME>`.
		/// e.g. ah@example.com, localhost
		///
		/// Does not include port, see [ServerConnectionOptions::ssh_port]
		#[arg(long, env = "YSURREAL_SSH_NAME")]
		pub ssh_name: String,

		#[arg(long, env = "YSURREAL_SSH_PORT")]
		pub ssh_port: String,
	}

	impl ServerConnectionOptions {
		pub fn full_name(&self) -> String {
			format!("{}:{}", self.ssh_name, self.ssh_port)
		}

		pub async fn connect(&self) -> Result<Session, openssh::Error> {
			Session::connect_mux(self.full_name(), openssh::KnownHosts::Strict).await
		}
	}
}
