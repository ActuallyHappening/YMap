use db::prelude::surrealdb_layers;
use leptos::either::Either;

use crate::prelude::*;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize)]
pub enum AppError {
  #[error("Waiting for database connection")]
  DbWaiting,

  #[error("Couldn't connect to database")]
  DbError(#[source] GenericError<db::Error>),

  #[error("Loading data from database ...")]
  DataLoading,

  #[error("Waiting for steam to be polled ...")]
  LiveQueryStreamWaiting,

  #[error("Waiting until next tick ...")]
  FirstTimeGlobalState,

  #[error("Couldn't load payload for thing with id {id}")]
  KnownRecordWrongPayload { id: ThingId },

  #[error("Please provide a valid record id")]
  CouldntParseRecordId {
    str: std::sync::Arc<str>,
    #[source]
    err: GenericError<surrealdb::Error>,
  },

  /// Won't render anything on error
  #[error("YOU SHOULD NEVER SEE THIS")]
  None,
}

impl IntoRender for &AppError {
  type Output = AnyView;

  fn into_render(self) -> Self::Output {
    if matches!(self, AppError::None) {
      return ().into_any();
    }
    let p = view! { <p> { self.to_string() } </p> };
    let pre = view! { <p> { format!("{:?}", self) } </p> };
    view! {<details class="error-hSDFKLJHhfhKLJDSFh732FH"> <summary>"An error occurred: " {p}</summary> {pre} </details>}
      .into_any()
  }
}

#[extension(pub trait ErrorExt)]
impl<F, T> F
where
  F: Fn() -> AppResult<T>,
{
  fn handle_error(self) -> impl Fn() -> Either<T, AnyView> {
    move || match self() {
      Ok(value) => Either::Left(value),
      Err(error) => Either::Right(error.into_render()),
    }
  }
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

#[component]
pub fn AppErrorBoundary(
  children: Children,
  #[prop(into, default = None)] name: Option<&'static str>,
) -> impl IntoView {
  let fallback = move |errors: ArcRwSignal<Errors>| {
    move || {
      errors
        .read()
        .iter()
        .map(|(_id, err)| err.clone().into_inner())
        .map(|err| match err.downcast_ref::<AppError>() {
          None => leptos::either::Either::Left({
            let ty = std::any::type_name_of_val(&err);
            error!(?err, ?ty, ?name, "Handling an unknown error case!");
            view! { <p style="color: red;">"An unknown error occurred"</p> }
          }),
          Some(err) => leptos::either::Either::Right(err.into_render()),
        })
        .collect_view()
    }
  };
  view! { <leptos::error::ErrorBoundary fallback>{children()}</leptos::error::ErrorBoundary> }
}
