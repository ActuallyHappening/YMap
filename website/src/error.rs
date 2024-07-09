use yauth::error::AuthError;

use crate::prelude::*;

#[derive(Clone, Debug, thiserror::Error, Deserialize, Serialize)]
pub enum AppError {
	#[error("Page not found")]
	NotFound,

	#[error("There was a problem talking to the backend: {0}")]
	SurrealError(GenericError),

	#[error("There was an error authenticating: {0}")]
	AuthError(GenericError),
}

impl From<yauth::error::AuthError> for AppError {
	fn from(value: AuthError) -> Self {
		AppError::AuthError(GenericError::new(&value))
	}
}

impl From<surrealdb::Error> for AppError {
	fn from(value: surrealdb::Error) -> Self {
		AppError::SurrealError(GenericError::new(&value))
	}
}
impl AppError {
	pub fn status_code(&self) -> StatusCode {
		match self {
			AppError::NotFound => StatusCode::NOT_FOUND,
			AppError::AuthError(_) => StatusCode::UNAUTHORIZED,
			AppError::SurrealError(_) => StatusCode::INTERNAL_SERVER_ERROR,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[error("{{ display: {display:?}, debug: {debug:?}, pretty_debug: {pretty_debug:?} }}")]
pub struct GenericError {
	display: String,
	debug: String,
	pretty_debug: String,
}

impl GenericError {
	fn new<T: std::fmt::Debug + std::fmt::Display>(error: &T) -> Self {
		Self {
			display: error.to_string(),
			debug: format!("{:?}", error),
			pretty_debug: format!("{:#?}", error),
		}
	}
}
