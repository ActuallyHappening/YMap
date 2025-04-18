use crate::prelude::*;
use db::prelude::surrealdb_layers;
use generic_err::GenericError;

#[component]
pub fn AppErrorBoundary(children: Element) -> Element {
  let handle_error = |errors: ErrorContext| errors.show().unwrap_or(rsx! { p { "{errors:?}"}});
  rsx! {
    ErrorBoundary {
      handle_error: handle_error,
      { children }
    }
  }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum AppError {
  /// Use on `use_resource` when its an `Option::None`
  #[error("Waiting for {0}")]
  Waiting(&'static str),

  #[error("Couldn't connect to database: {0}")]
  CouldntConnect(#[source] GenericError<db::Error>),
}

impl From<db::Error> for AppError {
  fn from(err: db::Error) -> Self {
    AppError::CouldntConnect(GenericError::from(err))
  }
}

impl From<surrealdb_layers::Error> for AppError {
  fn from(err: surrealdb_layers::Error) -> Self {
    AppError::CouldntConnect(GenericError::from(err.into()))
  }
}
