pub mod prelude {
  #![allow(unused_imports)]

  pub(crate) use std::collections::HashMap;
  pub(crate) use std::fmt::{Debug, Display};
  pub(crate) use std::hash::Hash;
  pub(crate) use std::marker::PhantomData;
  pub(crate) use std::str::FromStr;

  pub(crate) use serde::{Deserialize, Serialize};
  pub(crate) use url::Url;

  pub(crate) use utils::prelude::*;

  pub use crate::layers as surrealdb_layers;
  pub(crate) use surrealdb_layers::prelude::*;

  pub use crate::db::Db;
}

pub mod error;
pub mod layers;
pub mod thing;

pub use db::Db;
pub mod db;
