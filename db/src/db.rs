//! Specific to YMap abstractions

use crate::prelude::*;

#[derive(Clone)]
pub struct Db<Auth> {
  db: Surreal<Any>,
  phantom: PhantomData<Auth>,
}

impl<Auth> surrealdb_layers::GetDb for Db<Auth> {
  fn get_db(&self) -> &Surreal<Any> {
    &self.db
  }
}

pub mod creds {
  //! Creds are what gets you your authentication

  use crate::prelude::*;

  use super::auth;

  pub struct NoCreds;

  impl surrealdb_layers::Creds for NoCreds {
    type Auth = auth::NoAuth;

    async fn signin(&self, db: &Surreal<Any>) -> Result<Self::Auth, surrealdb::Error> {
      db.invalidate().await?;
      Ok(auth::NoAuth)
    }
  }

  #[allow(dead_code)]
  pub struct User {
    email: String,
    plaintext_password: String,
  }

  impl surrealdb_layers::Creds for User {
    type Auth = auth::User;

    async fn signin(&self, _db: &Surreal<Any>) -> Result<Self::Auth, surrealdb::Error> {
      // https://surrealdb.com/docs/surrealdb/security/authentication#record-users
      todo!()
    }
  }
}

pub mod auth {
  //! Auth is the current users session

  use crate::prelude::*;

  use surrealdb::{Surreal, engine::any::Any, opt::auth::Jwt};

  /// Public db
  #[derive(Clone)]
  pub struct NoAuth;

  impl surrealdb_layers::Auth for NoAuth {
    async fn authenticate(&self, db: &Surreal<Any>) -> Result<Self, surrealdb::Error> {
      db.invalidate().await?;
      Ok(NoAuth)
    }
  }

  /// Get acutal info from session
  #[derive(Clone)]
  pub struct User(Jwt);

  impl surrealdb_layers::Auth for User {
    async fn authenticate(&self, db: &Surreal<Any>) -> Result<Self, surrealdb::Error> {
      db.authenticate(self.0.clone()).await?;
      Ok(User(self.0.clone()))
    }
  }
}

pub mod conn {
  use std::pin::Pin;

  use crate::{error::Error, prelude::*};

  use super::{auth::NoAuth, creds::NoCreds};

  impl Db<()> {
    pub fn build() -> DbConnUrl {
      DbConnUrl { _priv: () }
    }
  }

  pub struct DbConnUrl {
    _priv: (),
  }

  impl surrealdb_layers::ConnBuilderUrl for DbConnUrl {
    type Next = Pin<Box<dyn Future<Output = Result<DbConnNsDb, Error>>>>;

    fn default_url(&self) -> Result<Url, surrealdb_layers::Error> {
      Ok("wss://eager-bee-06aqohg53hq27c0jg11k14gdbk.aws-use1.surreal.cloud".parse()?)
    }

    fn url(self, url: Url) -> Self::Next {
      Box::pin(async {
        surrealdb::engine::any::connect(url.to_string())
          .await
          .map(|conn| DbConnNsDb { conn })
          .map_err(|err| Error::CouldntConnectToUrl { url, err })
      })
    }
  }

  /// Connected
  pub struct DbConnNsDb {
    conn: Surreal<Any>,
  }

  impl DbConnNsDb {
    pub async fn prod(self) -> Result<DbConnCreds, Error> {
      let ns = "ymap";
      let db = "prod";
      self
        .conn
        .use_ns(ns)
        .use_db(db)
        .await
        .map_err(|err| Error::CouldntUseNsDb {
          ns: ns.to_owned(),
          db: db.to_owned(),
          err,
        })?;
      Ok(DbConnCreds { conn: self.conn })
    }
  }

  pub struct DbConnCreds {
    conn: Surreal<Any>,
  }

  impl DbConnCreds {
    pub fn public(self) -> Db<NoAuth> {
      Db {
        db: self.conn,
        phantom: PhantomData,
      }
    }
  }
}
