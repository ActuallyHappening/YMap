use generic_err::GenericError;

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
	let non_clone_debug = format!("{:?}", non_clone);

	let now_cloneable: Result<(), GenericError<NonCloneError>> =
		non_clone.map_err(GenericError::from);
	let cloneable_debug = format!("{:?}", now_cloneable);

	assert_eq!(non_clone_debug, cloneable_debug);

	// See? Magic!
	literally_everyday(now_cloneable);
}
