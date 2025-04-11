use db::prelude::surrealdb_layers;

use crate::prelude::*;

#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize)]
pub enum AppError {
  #[error("Waiting for database connection")]
  DbWaiting,

  #[error("Couldn't connect to database")]
  DbError(#[source] GenericError<db::Error>),

  #[error("Loading data from database ...")]
  DataLoading,
}

impl From<db::Error> for AppError {
  fn from(error: db::Error) -> Self {
    Self::DbError(GenericError::from(error))
  }
}

impl From<&db::Error> for AppError {
  fn from(error: &db::Error) -> Self {
    Self::DbError(GenericError::from_ref(error))
  }
}

impl From<surrealdb_layers::Error> for AppError {
  fn from(value: surrealdb_layers::Error) -> Self {
    Self::DbError(GenericError::from(db::Error::Inner(value)))
  }
}
