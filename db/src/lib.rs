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

  pub use crate::db::Db;
}

pub mod layers;
pub mod thing;

pub use db::Db;
pub mod db {
  //! Specific to YMap abstractions

  use surrealdb::{Surreal, engine::any::Any};

  use crate::prelude::*;

  pub struct Db<Auth> {
    db: Surreal<Any>,
    phantom: PhantomData<Auth>,
  }

  impl<Auth> surrealdb_layers::GetDb for Db<Auth> {
    fn get_db(&self) -> &Surreal<Any> {
      &self.db
    }
  }

  impl Db<()> {
    pub fn build() -> DbConnUrl {
      DbConnUrl { _priv: () }
    }
  }

  pub struct DbConnUrl {
    _priv: (),
  }

  impl surrealdb_layers::ConnBuilderUrl for DbConnUrl {
    type Next = DbConnNsDb;

    fn default_url(&self) -> Result<Url, surrealdb_layers::Error> {
      Ok("wss://eager-bee-06aqohg53hq27c0jg11k14gdbk.aws-use1.surreal.cloud".parse()?)
    }

    fn url(self, url: Url) -> Self::Next {
      DbConnNsDb { url }
    }
  }

  pub struct DbConnNsDb {
    url: Url,
  }

  impl DbConnNsDb {
    pub fn prod(self) -> DbConnBuilder {
      DbConnBuilder {
        url: self.url,
        ns: "ymap".to_string(),
        db: "prod".to_string(),
      }
    }
  }

  pub struct DbConnBuilder {
    url: Url,
    ns: String,
    db: String,
  }

  impl surrealdb_layers::DbConnBuilder for DbConnBuilder {
    fn get_ns(&self) -> impl Into<String> {
      &self.ns
    }

    fn get_db(&self) -> impl Into<String> {
      &self.db
    }

    fn get_url(&self) -> &Url {
      &self.url
    }
  }
}
