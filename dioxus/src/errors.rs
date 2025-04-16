#[derive(Debug, thiserror::Error, Clone)]
enum AppError {
  #[error("Couldn't connect to database: {0}")]
  CouldntConnect(#[source] GenericError<surrealdb::Error>),
}
