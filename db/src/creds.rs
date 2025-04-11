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
