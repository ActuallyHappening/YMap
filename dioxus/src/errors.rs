use crate::prelude::*;
use db::prelude::surrealdb_layers;
use generic_err::GenericError;

#[component]
pub fn AppErrorBoundary(children: Element) -> Element {
  let handle_error = |errors: ErrorContext| {
    error!("AppErrorBoundary: {:?}", errors);
    errors.show().unwrap_or(rsx! { p { "{errors:?}"}})
  };
  rsx! {
    ErrorBoundary {
      handle_error: handle_error,
      { children }
    }
  }
}

#[component]
pub fn AppSuspenseBoundary(children: Element) -> Element {
  let fallback = |_fallback| rsx! { p { "Loading ..."} };
  rsx! {
    SuspenseBoundary {
      fallback: fallback,
      children
    }
  }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error, Clone)]
pub enum AppError {
  #[error("Db conn: {0}")]
  DbConn(components::db::DbConnError),

  #[error("Couldn't connect to database: {0}")]
  CouldntConnect(#[source] GenericError<db::Error>),

  #[error("Thing with id {0} doesn't exist")]
  ThingDoesntExist(ThingId),

  #[error("Couldn't parse route key: {0}")]
  ParseRouteKey(#[source] GenericError<surrealdb::Error>),
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
