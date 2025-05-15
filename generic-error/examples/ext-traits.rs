use generic_err::GenericError;
use generic_err::Untyped;
use generic_err::prelude::*;

#[derive(Debug, thiserror::Error)]
enum NonCloneError {
	#[error("Database is not connected")]
	DbNotConnected,
}

/// Send to DB, clone a bit
fn literally_everyday<T>(_stuff: T)
where
	T: Clone + serde::Serialize + serde::de::DeserializeOwned,
{
	todo!()
}

fn main() {
	let non_clone: Result<(), NonCloneError> = Err(NonCloneError::DbNotConnected);
	let clonable: Result<(), GenericError<NonCloneError>> = non_clone.make_generic();

	// See? Magic!
	literally_everyday(clonable);

	let non_clonable: Result<(), NonCloneError> = Ok(());
	let clonable_untyped: Result<(), GenericError<Untyped>> = non_clonable.make_generic_untyped();

	literally_everyday(clonable_untyped);
}
