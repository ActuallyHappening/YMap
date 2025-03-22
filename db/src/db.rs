use surrealdb::Surreal;
use surrealdb::engine::any::Any;

use crate::auth;

pub type DbInner = Surreal<Any>;

/// Cheap to clone
#[derive(Clone)]
pub struct Db<Auth> {
  pub(crate) db: DbInner,
  pub(crate) auth: Auth,
}

impl<Auth> Db<Auth> {
  pub fn auth(&self) -> &Auth {
    &self.auth
  }
}

pub(crate) trait GetDb {
  fn db(&self) -> DbInner;
}

impl<Auth> GetDb for Db<Auth> {
  fn db(&self) -> DbInner {
    self.db.clone()
  }
}

impl Db<auth::Root> {
  pub async fn export(&self, path: impl AsRef<camino::Utf8Path>) -> Result<(), surrealdb::Error> {
    self.db.export(path.as_ref()).await
  }

  pub async fn import(&self, path: impl AsRef<camino::Utf8Path>) -> Result<(), surrealdb::Error> {
    self.db.import(path.as_ref()).await
  }
}

impl Db<auth::User> {
  /// Should be safe, assuming that signing in
  /// doesn't restrict any permissions
  pub fn downgrade(self) -> Db<auth::NoAuth> {
    Db {
      db: self.db,
      auth: auth::NoAuth,
    }
  }
}
