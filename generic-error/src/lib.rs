pub mod prelude {
  pub use crate::{GenericErrorExt as _, GenericErrorRefExt as _};
}

use extension_traits::extension;
use std::{convert::Infallible, fmt::Display, marker::PhantomData, sync::Arc};

/// A marker for [`GenericError`] that indicates it doesn't have
/// knowledge of the type it is wrapping
pub type Untyped = Infallible;

/// Wrapper around any error T such that it is `Clone`,
/// [`serde::Serialize`], and [`serde::Deserialize`] by storing
/// its initial `Debug` and `Display` representations as `String`s.
// TODO: Make an enum that maintains source from [core::error::Error]
// when not over a serialization/deserialization boundary
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenericError<T>
where
  T: 'static,
{
  display: String,
  debug: String,
  source: Option<Arc<GenericError<Untyped>>>,
  _phantom: PhantomData<T>,
}

#[extension(pub trait GenericErrorExt)]
impl<T, E> Result<T, E>
where
  E: std::fmt::Display + std::fmt::Debug,
{
  fn make_generic(self) -> Result<T, GenericError<E>>
  where
    E: core::error::Error,
  {
    self.map_err(GenericError::from)
  }

  fn make_generic_untyped(self) -> Result<T, GenericError<Untyped>>
  where
    E: core::error::Error + 'static,
  {
    self.make_generic().map_err(GenericError::untyped)
    // self.map_err(|err: E| GenericError::from_ref(&err).untyped())
  }
}

#[extension(pub trait GenericErrorRefExt)]
impl<T, E> Result<T, &E>
where
  E: Display + std::fmt::Debug + core::error::Error,
{
  fn make_generic_ref(self) -> Result<T, GenericError<E>> {
    self.map_err(|e| GenericError::from_ref(e))
  }

  fn make_generic_ref_untyped(self) -> Result<T, GenericError<Untyped>>
  where
    E: 'static,
  {
    self.make_generic_ref().map_err(GenericError::untyped)
  }
}

impl<T> core::error::Error for GenericError<T> {}

impl<T> std::fmt::Debug for GenericError<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.debug)
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
      source: self.source.clone(),
      _phantom: PhantomData,
    }
  }
}

impl<T> GenericError<T> {
  /// Preserves source
  pub fn untyped(self) -> GenericError<Untyped> {
    GenericError {
      display: self.display,
      debug: self.debug,
      source: self.source,
      _phantom: PhantomData,
    }
  }
}

impl GenericError<Untyped> {
  fn from_dyn(err: &dyn core::error::Error) -> GenericError<Untyped> {
    GenericError {
      display: err.to_string(),
      debug: format!("{:?}", err),
      source: err
        .source()
        .map(|source| Arc::new(GenericError::from_dyn(source))),
      _phantom: PhantomData,
    }
  }
}

impl<E> GenericError<E>
where
  E: std::fmt::Display + std::fmt::Debug + core::error::Error,
{
  /// Preserves source
  pub fn from(err: E) -> GenericError<E> {
    Self {
      debug: format!("{:?}", err),
      display: err.to_string(),
      source: err
        .source()
        .map(|source: &dyn core::error::Error| Arc::new(GenericError::from_dyn(source).untyped())),
      _phantom: PhantomData,
    }
  }

  /// Preserves source
  pub fn from_ref(err: &E) -> GenericError<E> {
    Self {
      debug: format!("{:?}", err),
      display: err.to_string(),
      source: err
        .source()
        .map(|err| Arc::new(GenericError::from_dyn(err))),
      _phantom: PhantomData,
    }
  }
}

impl<E> GenericError<E>
where
  E: std::fmt::Display + std::fmt::Debug,
{
  pub fn from_non_err(err: E) -> GenericError<E> {
    Self {
      debug: format!("{:?}", err),
      display: err.to_string(),
      source: None,
      _phantom: PhantomData,
    }
  }

  pub fn from_non_err_ref(err: &E) -> GenericError<E> {
    Self {
      debug: format!("{:?}", err),
      display: err.to_string(),
      source: None,
      _phantom: PhantomData,
    }
  }
}
