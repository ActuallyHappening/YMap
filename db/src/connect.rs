use url::Url;

use crate::{
  creds::{self, DbCreds},
  prelude::*,
};

impl Db<creds::Guest> {
  pub fn connect_wss() -> DbConnectBuilder<creds::Guest> {
    DbConnectBuilder::new(routes::db_wss())
  }

  pub fn connect_https() -> DbConnectBuilder<creds::Guest> {
    DbConnectBuilder::new(routes::db_https())
  }
}

pub struct DbConnectBuilder<Creds> {
  url: Url,
  creds: Creds,
  ns: &'static str,
  db: &'static str,
}

impl<Creds> DbConnectBuilder<Creds> {
  pub fn ns(&self) -> &str {
    self.ns
  }

  pub fn db(&self) -> &str {
    self.db
  }
}

impl DbConnectBuilder<creds::Guest> {
  fn new(url: Url) -> Self {
    Self {
      url,
      creds: creds::Guest,
      ns: crate::NS,
      db: crate::DB,
    }
  }

  fn with_creds<N>(self, creds: N) -> DbConnectBuilder<N> {
    DbConnectBuilder {
      url: self.url,
      creds,
      ns: self.ns,
      db: self.db,
    }
  }

  pub fn root(self, creds: creds::Root) -> DbConnectBuilder<creds::Root> {
    self.with_creds(creds)
  }

  pub fn user(self) -> UserConnectBuilder {
    UserConnectBuilder(self)
  }
}

pub struct UserConnectBuilder(DbConnectBuilder<creds::Guest>);

impl UserConnectBuilder {
  pub fn signup(
    self,
    creds: crate::users::SignUpUser,
  ) -> DbConnectBuilder<crate::users::SignUpUser> {
    self.0.with_creds(creds)
  }

  pub fn signin(
    self,
    creds: crate::users::SignInUser,
  ) -> DbConnectBuilder<crate::users::SignInUser> {
    self.0.with_creds(creds)
  }

  pub fn authenticate(
    self,
    creds: creds::AuthenticateUser,
  ) -> DbConnectBuilder<creds::AuthenticateUser> {
    self.0.with_creds(creds)
  }
}

impl<Creds> DbConnectBuilder<Creds>
where
  Creds: DbCreds,
{
  pub async fn finish(self) -> Result<Db<<Creds as DbCreds>::Auth>, ConnectErr> {
    let db = surrealdb::engine::any::connect(self.url.to_string())
      .await
      .map_err(ConnectErr::CouldntConnect)?;
    db.use_ns(self.ns).await.map_err(ConnectErr::CouldntUseNs)?;
    db.use_db(self.db).await.map_err(ConnectErr::CouldntUseDb)?;

    let creds_debug = format!("{:?}", &self.creds);
    let auth = self
      .creds
      .authenticate(db.clone(), self.ns, self.db)
      .await
      .map_err(|err| ConnectErr::CouldntAuthenticate { err, creds_debug })?;

    // that fateful line
    Ok(Db { db, auth })
  }
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectErr {
  #[error("Couldn't connect to the database (network)")]
  CouldntConnect(#[source] surrealdb::Error),

  #[error("Couldn't connect to the database (ns)")]
  CouldntUseNs(#[source] surrealdb::Error),

  #[error("Couldn't connect to the database (db)")]
  CouldntUseDb(#[source] surrealdb::Error),

  #[error("Couldn't connect to the database (auth)")]
  CouldntAuthenticate {
    #[source]
    err: surrealdb::Error,
    creds_debug: String,
  },
}
