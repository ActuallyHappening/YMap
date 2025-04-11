use std::pin::Pin;

use crate::{error::Error, prelude::*};

use super::auth::NoAuth;

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
