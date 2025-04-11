use crate::prelude::*;

#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize)]
pub enum Error {
  #[error("Waiting for database connection")]
  DbWaiting,

  #[error("Couldn't connect to database")]
  DbError(#[source] GenericError<db::Error>),
}

impl From<db::Error> for Error {
  fn from(error: db::Error) -> Self {
    Self::DbError(GenericError::from(error))
  }
}

impl From<surrealdb_layers::Error> for Error {
  fn from(value: surrealdb_layers::Error) -> Self {
    Self::DbError(GenericError::from(db::Error::Inner(value)))
  }
}
