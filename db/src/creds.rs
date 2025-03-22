//! Creds = initially signing into db

use surrealdb::{Surreal, engine::any::Any, opt::auth::Jwt};

use crate::{auth, prelude::*};

#[allow(private_bounds)]
pub trait DbCreds: Sealed + Debug {
  type Auth;

  async fn authenticate(
    self,
    conn: Surreal<Any>,
    ns: &str,
    db: &str,
  ) -> Result<Self::Auth, surrealdb::Error>;
}
trait Sealed {}

#[derive(Debug)]
pub struct Guest;

impl Sealed for Guest {}

impl DbCreds for Guest {
  type Auth = auth::NoAuth;

  async fn authenticate(
    self,
    _conn: Surreal<Any>,
    _ns: &str,
    _db: &str,
  ) -> Result<Self::Auth, surrealdb::Error> {
    Ok(auth::NoAuth {})
  }
}

pub struct Root(surrealdb::opt::auth::Root<'static>);

impl Debug for Root {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Root")
      .field("user", &self.0.username)
      .field("pass", &"<redacted>")
      .finish()
  }
}

impl Sealed for Root {}
impl DbCreds for Root {
  type Auth = auth::Root;

  async fn authenticate(
    self,
    conn: Surreal<Any>,
    _ns: &str,
    _db: &str,
  ) -> Result<auth::Root, surrealdb::Error> {
    conn.signin(self.0).await?;
    Ok(auth::Root {})
  }
}

#[cfg(feature = "root-creds")]
impl Root {
  /// Requires env/db-root-creds
  pub fn new() -> Self {
    Self(surrealdb::opt::auth::Root {
      username: "root",
      password: env::db::ROOT_PASS,
    })
  }
}

impl Sealed for crate::users::SignUpUser {}
impl DbCreds for crate::users::SignUpUser {
  type Auth = auth::User;

  async fn authenticate(
    self,
    db: Surreal<Any>,
    _ns: &str,
    _db: &str,
  ) -> Result<Self::Auth, surrealdb::Error> {
    let jwt = db
      .signup(surrealdb::opt::auth::Record {
        namespace: _ns,
        database: _db,
        access: crate::users::AUTH_ACCESS,
        params: self,
      })
      .await?;
    Ok(auth::User { jwt })
  }
}

impl Sealed for crate::users::SignInUser {}
impl DbCreds for crate::users::SignInUser {
  type Auth = auth::User;

  async fn authenticate(
    self,
    conn: Surreal<Any>,
    ns: &str,
    db: &str,
  ) -> Result<auth::User, surrealdb::Error> {
    let jwt = conn
      .signin(surrealdb::opt::auth::Record {
        namespace: ns,
        database: db,
        access: crate::users::AUTH_ACCESS,
        params: self,
      })
      .await?;
    Ok(auth::User { jwt })
  }
}

#[derive(Clone, Debug)]
pub struct AuthenticateUser(pub Jwt);

impl Sealed for AuthenticateUser {}
impl DbCreds for AuthenticateUser {
  type Auth = auth::User;

  async fn authenticate(
    self,
    conn: Surreal<Any>,
    _ns: &str,
    _db: &str,
  ) -> Result<Self::Auth, surrealdb::Error> {
    conn.authenticate(self.0.clone()).await?;
    Ok(auth::User { jwt: self.0 })
  }
}
