pub mod prelude {
	pub(crate) use camino::Utf8PathBuf;
	pub(crate) use color_eyre::eyre::{Report, WrapErr};
	pub(crate) use rand::Rng;
	pub(crate) use std::future::{Future, IntoFuture};
	pub(crate) use surrealdb::engine::any::Any;
	pub(crate) use surrealdb::{Connection, Surreal};
	pub(crate) use tracing::*;
	#[cfg(not(target_arch = "wasm32-unknown-unknown"))]
	pub(crate) use which::which;

	// public exports
	pub use crate::config::{DBConnectRemoteConfig, DBRootCredentials, DBStartConfig};
	#[cfg(not(target_arch = "wasm32-unknown-unknown"))]
	pub use crate::testing::{start_testing_db, TestingMemoryDB};
}

pub mod config;
pub mod configs;
#[cfg(not(target_arch = "wasm32-unknown-unknown"))]
pub mod testing;
