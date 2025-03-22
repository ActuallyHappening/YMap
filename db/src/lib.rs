#![allow(async_fn_in_trait)]

pub mod prelude {
  #![allow(unused_imports)]

  pub(crate) use std::fmt::{Debug, Display};
  pub(crate) use std::num::NonZero;
  pub(crate) use std::ops::Deref;

  pub(crate) use extension_traits::extension;
  pub(crate) use nonzero_lit::u8;
  pub(crate) use serde::{Deserialize, Serialize};
  pub use tokio_stream::StreamExt as _;
  pub(crate) use tracing::{debug, error, info, trace, warn};

  pub(crate) use crate::common::*;
  pub(crate) use crate::db::GetDb;

  pub use crate::Db;
  pub use crate::common::TableDescriptor as _;
  pub use crate::select::LiveSelectTable as _;
}

pub(crate) const NS: &str = "jyd";
pub(crate) const DB: &str = "prod";

pub use db::{Db, DbInner};

pub mod auth;
pub mod cartridges;
pub mod common;
pub mod connect;
pub mod creds;
pub mod db;
pub mod errors;
pub mod inventory;
pub mod invoice;
pub mod orders;
pub mod reviews;
pub mod search;
pub mod select;
pub mod sendle;
pub mod support;
pub mod users;
