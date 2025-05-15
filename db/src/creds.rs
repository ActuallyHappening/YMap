//! Creds are what gets you your authentication.
//!
//! TODO: Edge case of multiple users with the same email?
//! Maybe we go by user ID not email as the identifier?

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

#[derive(Serialize)]
pub struct SignInUser {
	email: String,
	#[serde(rename = "password")]
	plaintext_password: String,
}

impl surrealdb_layers::Creds for SignInUser {
	type Auth = auth::User;

	async fn signin(&self, db: &Surreal<Any>) -> Result<Self::Auth, surrealdb::Error> {
		let jwt = db
			.signin(surrealdb::opt::auth::Record {
				// todo: generalize this a different way
				namespace: "ymap",
				database: "prod",
				access: "user",
				params: &self,
			})
			.await?;

		Ok(auth::User::new(jwt))
	}
}

#[derive(Serialize)]
pub struct SignUpUser {
	name: String,
	email: String,
	#[serde(rename = "password")]
	plaintext_password: String,
}

impl surrealdb_layers::Creds for SignUpUser {
	type Auth = auth::User;

	async fn signin(&self, db: &Surreal<Any>) -> Result<Self::Auth, surrealdb::Error> {
		let jwt = db
			.signup(surrealdb::opt::auth::Record {
				// todo: generalize this a different way
				namespace: "ymap",
				database: "prod",
				access: "user",
				params: &self,
			})
			.await?;

		Ok(auth::User::new(jwt))
	}
}
