//! Specific to YMap abstractions

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

pub mod errors {
  use crate::prelude::*;

  #[derive(thiserror::Error, Debug)]
  pub enum Error {
    #[error("[db] {0}")]
    Layers(#[from] surrealdb_layers::Error),

    #[error("[db] Couldn't authenticate: {0}")]
    CouldntAuthenticate(#[source] surrealdb::Error),
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
  pub struct NoAuth;

  impl surrealdb_layers::Auth for NoAuth {
    async fn authenticate(&self, db: &Surreal<Any>) -> Result<Self, surrealdb::Error> {
      db.invalidate().await?;
      Ok(NoAuth)
    }
  }

  /// Get acutal info from session
  pub struct User(Jwt);

  impl surrealdb_layers::Auth for User {
    async fn authenticate(&self, db: &Surreal<Any>) -> Result<Self, surrealdb::Error> {
      db.authenticate(self.0.clone()).await?;
      Ok(User(self.0.clone()))
    }
  }
}

pub mod conn {
  use crate::prelude::*;

  use super::creds::NoCreds;

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
    pub fn prod(self) -> DbConnCreds {
      DbConnCreds {
        url: self.url,
        ns: "ymap".to_string(),
        db: "prod".to_string(),
      }
    }
  }

  pub struct DbConnCreds {
    url: Url,
    ns: String,
    db: String,
  }

  impl DbConnCreds {
    pub fn public(self) -> DbConnBuilder<NoCreds> {
      DbConnBuilder {
        url: self.url,
        ns: self.ns,
        db: self.db,
        creds: NoCreds,
      }
    }
  }

  pub struct DbConnBuilder<Creds> {
    url: Url,
    ns: String,
    db: String,
    creds: Creds,
  }

  impl<Creds> surrealdb_layers::DbConnBuilder for DbConnBuilder<Creds>
  where
    Creds: surrealdb_layers::Creds,
  {
    type Next = Db<<Creds as surrealdb_layers::Creds>::Auth>;

    async fn db_authenticate(
      &self,
      conn: Surreal<Any>,
    ) -> Result<Self::Next, surrealdb_layers::Error> {
      self
        .creds
        .signin(&conn)
        .await
        .map_err(surrealdb_layers::Error::CouldntAuthenticate)?;

      Ok(Db {
        db: conn,
        phantom: PhantomData,
      })
    }

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
