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

impl User {
  pub(crate) fn new(jwt: Jwt) -> Self {
    User(jwt)
  }
}

impl surrealdb_layers::Auth for User {
  async fn authenticate(&self, db: &Surreal<Any>) -> Result<Self, surrealdb::Error> {
    db.authenticate(self.0.clone()).await?;
    Ok(User(self.0.clone()))
  }
}
