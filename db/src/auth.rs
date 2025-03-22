//! Auth = info stored after signing in

use crate::prelude::*;
use surrealdb::opt::auth::Jwt;

/// [crate::creds::Guest`]
#[derive(Debug, Clone)]
pub struct NoAuth;

/// [crate::creds::Root`]
#[derive(Debug, Clone)]
pub struct Root;

#[derive(Debug, Clone)]
pub struct User {
  pub(crate) jwt: Jwt,
}

impl User {
  pub fn jwt(&self) -> Jwt {
    self.jwt.clone()
  }
}
