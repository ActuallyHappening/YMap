pub mod prelude {
	pub(crate) use surrealdb::{Connection, Surreal};
	pub(crate) use tracing::*;
	pub(crate) use serde::{Deserialize, Serialize};

	pub use crate::connection::*;
}

mod connection;
