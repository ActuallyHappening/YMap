#![warn(missing_debug_implementations, rust_2018_idioms)]

pub mod prelude {
	// deps re-exports
	pub(crate) use garde::Validate;
	pub(crate) use serde::{Deserialize, Serialize};
	pub(crate) use surrealdb::{Connection, Surreal};
	pub(crate) use tracing::*;

	// internal re-exports
	pub(crate) use crate::auth_connection::AuthConnection;
	pub(crate) use crate::types::{UserRecord, ValidatedType, ValidationError};

	// public exports
	pub use crate::commands::*;
	pub use crate::config::DBAuthConfig;
	pub use crate::error::AuthError;
	pub use std::str::FromStr;
	pub use surrealdb::opt::auth::Jwt;
	pub use ysurreal::prelude::*;
}

pub mod auth_connection;
pub(crate) mod commands;
pub mod config;
pub mod configs;
pub mod error;
pub mod types;
