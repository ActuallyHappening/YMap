use crate::prelude::*;

#[derive(Debug, thiserror::Error, Clone)]
pub enum AppError {
  #[error("Waiting for database connection")]
  DbWaiting,

  #[error("Loading data from database ...")]
  DataLoading,

  #[error("Waiting for steam to be polled ...")]
  LiveQueryStreamWaiting,

  #[error("Waiting until next tick ...")]
  FirstTimeGlobalState,

  #[error("Surreal custom: {0}")]
  Surreal(#[source] GenericError<surrealdb::Error>),
}

impl From<surrealdb::Error> for AppError {
  fn from(error: surrealdb::Error) -> Self {
    Self::Surreal(GenericError::from(error))
  }
}

impl IntoRender for &AppError {
  type Output = AnyView;

  fn into_render(self) -> Self::Output {
    let p = view! { <p> { self.to_string() } </p> };
    let pre = view! { <pre> { format!("{:?}", self) } </pre> };
    (p, pre).into_any()
  }
}
