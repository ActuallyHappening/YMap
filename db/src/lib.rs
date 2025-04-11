//! Specific to YMap abstractions

#![allow(async_fn_in_trait)]

pub mod prelude {
  #![allow(unused_imports)]

  pub(crate) use std::collections::HashMap;
  pub(crate) use std::fmt::{Debug, Display};
  pub(crate) use std::hash::Hash;
  pub(crate) use std::marker::PhantomData;
  pub(crate) use std::str::FromStr;

  pub(crate) use extension_traits::extension;
  pub(crate) use serde::{Deserialize, Serialize};
  pub(crate) use url::Url;

  pub(crate) use crate::Error;
  pub(crate) use utils::prelude::*;

  pub use surrealdb_layers;
  pub use surrealdb_layers::prelude::*;
  pub use thing::prelude::*;

  pub use crate::Db;
  pub use crate::things::ThingExt as _;
}

pub use error::Error;
pub mod error;

pub mod user;

mod things {
  use thing::well_known::KnownRecord;

  use crate::{auth, prelude::*};

  #[extension(pub trait ThingExt)]
  impl Db<auth::NoAuth> {
    async fn known_thing<P>(&self) -> Result<thing::Thing<P>, Error>
    where
      thing::Thing<P>: serde::de::DeserializeOwned + KnownRecord,
    {
      let id = <thing::Thing<P>>::known_id();
      let thing: Option<thing::Thing<P>> = self
        .db()
        .select(id.clone())
        .await
        .map_err(|err| Error::CouldntSelect(err))?;
      let thing = thing.ok_or(Error::KnownRecordNotFound(id.into_inner()))?;
      Ok(thing)
    }
  }
}

use crate::prelude::*;

#[derive(Clone)]
pub struct Db<Auth> {
  db: Surreal<Any>,
  auth: Auth,
}

impl<Auth> Db<Auth> {
  pub fn auth(&self) -> &Auth {
    &self.auth
  }
}

impl Db<auth::User> {
  pub fn downgrade(self) -> Db<auth::NoAuth> {
    Db {
      db: self.db,
      auth: auth::NoAuth,
    }
  }
}

impl<Auth> surrealdb_layers::GetDb for Db<Auth> {
  fn get_db(&self) -> &Surreal<Any> {
    &self.db
  }
}

pub mod auth;
pub mod conn;
pub mod creds;
