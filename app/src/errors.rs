use std::fmt::Display;

use crate::prelude::*;

use leptos::tachys;

pub use generic::*;

pub use app_error::*;
mod app_error {
  //! Error handling using leptos ErrorBoundary

  use leptos_router::params::ParamsError;

  use super::components::Pre;
  use crate::prelude::*;

  #[derive(Debug, thiserror::Error)]
  pub enum AppError {
    #[error("Database is loading: {0}")]
    DbConn(#[from] GenericError<crate::db::ConnectErr>),

    #[error("You must be logged in to do this")]
    MustBeLoggedIn,

    #[error("Loading ...")]
    LoadingContent,

    #[error("Error talking to backend server: {0}")]
    ServerFnError(ServerFnError),

    #[error("Invalid URL parameters: {0}")]
    ParamsError(#[from] ParamsError),

    #[error("Error placing review in db: {0}")]
    PlaceReviewErr(#[from] GenericError<db::reviews::PlaceReviewErr>),
  }

  impl From<ServerFnError> for AppError {
    fn from(err: ServerFnError) -> Self {
      Self::ServerFnError(err)
    }
  }

  impl AppError {
    pub fn flatten_server_fn<T, E>(res: Result<Result<T, E>, ServerFnError>) -> Result<T, AppError>
    where
      E: Into<AppError>,
    {
      match res {
        Ok(Ok(val)) => Ok(val),
        Ok(Err(err)) => Err(err.into()),
        Err(err) => Err(Self::ServerFnError(err)),
      }
    }
  }

  pub type AppRes<T> = Result<T, AppError>;

  impl IntoRender for AppError {
    type Output = AnyView;

    fn into_render(self) -> Self::Output {
      view! {
        <Pre err=GenericError::from_ref(&self) />
        <p>{self.to_string()}</p>
      }
      .into_any()
    }
  }
}

pub(crate) trait ReportErr<T> {
  fn report_err(self, msg: impl Display) -> Option<T>;
}

impl<T, E> ReportErr<T> for Result<T, E>
where
  E: std::fmt::Debug,
{
  fn report_err(self, msg: impl Display) -> Option<T> {
    match self {
      Err(err) => {
        error!(?err, message = msg.to_string());
        None
      }
      Ok(val) => Some(val),
    }
  }
}

#[extension(pub(crate) trait MapView)]
impl<T, E> Result<T, E>
where
  E: tachys::view::IntoRender,
{
  fn map_view<V: IntoView>(
    self,
    f: impl FnOnce(T) -> V,
  ) -> Either<V, <E as tachys::view::IntoRender>::Output> {
    match self {
      Ok(val) => Either::Left(f(val)),
      Err(err) => Either::Right(err.into_render()),
    }
  }
}

pub mod components {
  use crate::prelude::*;

  /// Will debug render an error, plus logging it.
  /// Will hide the error render impl if `cfg!(feature = "prod")`,
  /// so only debug builds see it
  #[component]
  pub fn Pre(err: impl std::fmt::Debug, #[prop(default = false)] hide: bool) -> impl IntoView {
    error!(?err);
    let show = !hide;
    match show & !cfg!(feature = "prod") {
      true => Either::Left(view! { <pre>{format!("{:?}", err)}</pre> }),
      false => Either::Right(()),
    }
  }
}

mod generic {
  use crate::prelude::*;

  use std::{fmt::Display, marker::PhantomData};

  pub struct Untyped;
  pub type GenericErr = GenericError<Untyped>;

  /// Wrapper around any error T such that it is `Clone`,
  /// [`serde::Serialize`], and [`serde::Deserialize`] by storing
  /// its initial `Debug` and `Display` representations as `String`s.
  // TODO: Make an enum that maintains source from [core::error::Error]
  // when not over a serialization/deserialization boundary
  #[derive(Serialize, Deserialize)]
  pub struct GenericError<T>
  where
    T: 'static,
  {
    display: String,
    debug: String,
    _phantom: PhantomData<T>,
  }

  #[extension(pub trait GenericErrorExt)]
  impl<T, E> Result<T, E>
  where
    E: std::fmt::Display + std::fmt::Debug,
  {
    fn err_generic(self) -> Result<T, GenericError<E>> {
      self.map_err(GenericError::from)
    }
  }

  #[extension(pub trait GenericErrorRefExt)]
  impl<T, E> Result<T, &E>
  where
    E: Display + std::fmt::Debug,
  {
    fn err_generic_ref(self) -> Result<T, GenericError<E>> {
      self.map_err(|e| GenericError::from_ref(e))
    }
  }

  impl<T> core::error::Error for GenericError<T> {}

  impl<T> std::fmt::Debug for GenericError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
        f,
        "GenericError::<{}>::({})",
        core::any::type_name::<T>(),
        self.debug
      )
    }
  }

  impl<T> std::fmt::Display for GenericError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.display)
    }
  }

  impl<T> Clone for GenericError<T> {
    fn clone(&self) -> Self {
      GenericError {
        display: self.display.clone(),
        debug: self.debug.clone(),
        _phantom: PhantomData,
      }
    }
  }

  impl<T> GenericError<T>
  where
    T: std::fmt::Display + std::fmt::Debug,
  {
    pub fn from(err: T) -> GenericError<T> {
      Self {
        debug: format!("{:?}", err),
        display: err.to_string(),
        _phantom: PhantomData,
      }
    }

    pub fn from_ref(err: &T) -> GenericError<T> {
      Self {
        debug: format!("{:?}", err),
        display: err.to_string(),
        _phantom: PhantomData,
      }
    }
  }

  impl GenericErr {
    pub fn from_generic<T>(err: T) -> GenericErr
    where
      T: std::fmt::Display + std::fmt::Debug,
    {
      Self {
        debug: format!("{:?}", err),
        display: err.to_string(),
        _phantom: PhantomData,
      }
    }
  }

  impl<T> From<T> for GenericError<T>
  where
    T: std::fmt::Display + std::fmt::Debug,
  {
    fn from(err: T) -> Self {
      Self {
        debug: format!("{:?}", err),
        display: err.to_string(),
        _phantom: PhantomData,
      }
    }
  }
}
