use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
	#[error("A validation error occurred: {0}")]
	ValidationError(#[from] ValidationError),

	#[error("Some internal invariant was broken: {0}")]
	InternalInvariantBroken(#[from] InternalInvariantBroken),

	#[error("An error occurred with the database: {0}")]
	SurrealError(#[from] surrealdb::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum InternalInvariantBroken {
	#[error(
		"User was signed in to the scope, but no corresponding record was found in the users table"
	)]
	UserSignedInButNoRecord,
	#[error(
		"User was signed up to the scope, but no corresponding record was found in the users table"
	)]
	UserSignedUpButNoRecord,
}
