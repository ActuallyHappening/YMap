pub mod prelude {
	// deps re-exports
	pub(crate) use surrealdb::{Connection, Surreal};
	pub(crate) use tracing::*;
	pub(crate) use serde::{Deserialize, Serialize};
	pub(crate) use garde::Validate;

	// internal re-exports
	pub(crate) use crate::types::{ValidatedType, ValidationError};

	// public exports
	pub use crate::{AuthError, AuthConnection};
}
use crate::prelude::*;

pub mod types;
pub mod signup;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
	#[error("A validation error occurred: {0}")]
	ValidationError(#[from] ValidationError),

	#[error("Some internal invariant was broken: {0}")]
	InternalInvariantBroken(String),

	#[error("An error occurred with the database: {0}")]
	SurrealError(#[from] surrealdb::Error),
}

#[derive(Debug, Validate)]
pub struct AuthConnection<'db, C: Connection> {
	#[garde(skip)]
	pub db: &'db Surreal<C>,

	#[garde(length(min = 1))]
	pub namespace: String,

	#[garde(length(min = 1))]
	pub database: String,

	#[garde(length(min = 1))]
	pub users_table: String,

	#[garde(length(min = 1))]
	pub scope: String,
}