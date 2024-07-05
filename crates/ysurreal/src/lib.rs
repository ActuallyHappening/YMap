pub mod prelude {
	pub(crate) use camino::Utf8PathBuf;
	pub(crate) use rand::Rng;
	pub(crate) use std::future::{Future, IntoFuture};
	pub(crate) use surrealdb::engine::any::Any;
	pub(crate) use surrealdb::Surreal;
	pub(crate) use tracing::*;

	// public exports
	pub use crate::config::{DBConnectRemoteConfig, DBRootCredentials, DBStartConfig};
}

pub mod config;

pub mod configs;
