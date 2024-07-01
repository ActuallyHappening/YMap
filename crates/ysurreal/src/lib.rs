pub mod prelude {
	pub(crate) use clap::Args;
	pub(crate) use tracing::*;

	pub use crate::args::SurrealConnectionOptions;
}

pub mod args {
	use clap::ValueEnum;
	use surrealdb::{engine::remote::ws::Ws, Surreal};

	use crate::prelude::*;

	#[derive(Args)]
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

	impl SurrealConnectionOptions {
		pub async fn connect(&self) -> Result<Surreal<Ws>, surrealdb::Error> {
			let db = Surreal::new(&self.connection).await?;
			db.use_ns(&self.namespace).use_db(&self.database).await?;

			Ok(db)
		}
	}
}
