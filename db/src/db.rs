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

pub mod creds {
  //! Creds are what gets you your authentication

  use crate::prelude::*;

  use super::auth;

  pub(super) trait Creds {
    type Auth: auth::Auth;

    async fn login(&self, db: &Surreal<Any>) -> Result<Self::Auth, surrealdb::Error>;
  }

  pub struct NoCreds;

  impl Creds for NoCreds {
    type Auth = auth::NoAuth;

    async fn login(&self, db: &Surreal<Any>) -> Result<Self::Auth, surrealdb::Error> {
      db.invalidate().await?;
      Ok(auth::NoAuth)
    }
  }

  #[allow(dead_code)]
  pub struct User {
    email: String,
    plaintext_password: String,
  }

  impl Creds for User {
    type Auth = auth::User;

    async fn login(&self, _db: &Surreal<Any>) -> Result<Self::Auth, surrealdb::Error> {
      todo!()
    }
  }
}

pub mod auth {
  //! Auth is the current users session

  use surrealdb::{Surreal, engine::any::Any, opt::auth::Jwt};

  pub(super) trait Auth: Sized {
    async fn authenticate(&self, db: &Surreal<Any>) -> Result<Self, surrealdb::Error>;
  }

  /// Public db
  pub struct NoAuth;

  impl Auth for NoAuth {
    async fn authenticate(&self, db: &Surreal<Any>) -> Result<Self, surrealdb::Error> {
      db.invalidate().await?;
      Ok(NoAuth)
    }
  }

  /// Get acutal info from session
  pub struct User(Jwt);

  impl Auth for User {
    async fn authenticate(&self, db: &Surreal<Any>) -> Result<Self, surrealdb::Error> {
      db.authenticate(self.0.clone()).await?;
      Ok(User(self.0.clone()))
    }
  }
}

pub mod conn {
  use crate::prelude::*;

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
