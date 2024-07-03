#![warn(missing_debug_implementations, rust_2018_idioms)]

pub mod prelude {
	// deps re-exports
	pub(crate) use garde::Validate;
	pub(crate) use serde::{Deserialize, Serialize};
	pub(crate) use surrealdb::{Connection, Surreal};
	pub(crate) use tracing::*;

	// internal re-exports
	pub(crate) use crate::types::{ValidatedType, ValidationError};

	// public exports
	pub use crate::{AuthConnection, AuthError};
	pub use ysurreal::prelude::*;
}
use color_eyre::eyre::{Context, Error};

use crate::prelude::*;

pub mod signup;
pub mod types;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
	#[error("A validation error occurred: {0}")]
	ValidationError(#[from] ValidationError),

	#[error("Some internal invariant was broken: {0}")]
	InternalInvariantBroken(String),

	#[error("An error occurred with the database: {0}")]
	SurrealError(#[from] surrealdb::Error),
}

#[derive(Debug)]
pub struct AuthConnection<'db, C: Connection> {
	pub db: &'db Surreal<C>,

	pub auth_instance: AuthInstance,
}

/// Options for what auth should be used.
#[derive(Validate, Debug)]
pub struct AuthInstance {
	#[garde(length(min = 1))]
	pub namespace: String,

	#[garde(length(min = 1))]
	pub database: String,

	#[garde(length(min = 1))]
	pub users_table: String,

	#[garde(length(min = 1))]
	pub scope: String,
}

// impl AuthInstance {
// 	/// For testing purposes, loads from the environment variables
// 	/// `YAUTH_USERS_TABLE` and `YAUTH_USERS_SCOPE`.
// 	pub fn from_env_testing() -> Self {
// 		use std::env::var;
// 		let conn = TestingDBConnection::from_env();
// 		AuthInstance {
// 			namespace: conn.namespace,
// 			database: conn.database,
// 			users_table: var("YAUTH_USERS_TABLE")
// 				.wrap_err("Couldn't load YAUTH_USERS_TABLE")
// 				.unwrap(),
// 			scope: var("YAUTH_USERS_SCOPE")
// 				.wrap_err("Couldn't load YAUTH_USERS_SCOPE")
// 				.unwrap(),
// 		}
// 	}
// }
