use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Waiting for database connection")]
  DbWaiting,

  #[error("Couldn't connect to database")]
  DbError(#[source] GenericError<db::Error>),
}
