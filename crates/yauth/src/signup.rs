use crate::prelude::*;
use crate::types::{Email, Password, UserRecord, Username};
use surrealdb::opt::auth::Scope;

/// User facing signup data request
#[derive(Debug, garde::Validate, Serialize)]
pub struct Signup {
	#[garde(dive)]
	pub username: Username,

	#[garde(dive)]
	pub password: Password,

	#[garde(dive)]
	pub email: Email,
}

impl Signup {
	pub fn new(username: String, password: String, email: String) -> Result<Self, ValidationError> {
		Ok(Signup {
			username: Username::try_new(username)?,
			password: Password::try_new(password)?,
			email: Email::try_new(email)?,
		})
	}
}

impl<'db, C: Connection> AuthConnection<'db, C> {
	#[instrument(skip_all)]
	pub async fn signup(&self, signup: Signup) -> Result<UserRecord, AuthError> {
		info!(
			message = "Signing up new user",
			username = ?signup.username,
			email = ?signup.email,
			users_table = ?self.users_table,
			scope = ?self.scope,
			database = ?self.database,
			namespace = ?self.namespace,
			note = "Errors are not reported on the same verbosity as this log"
		);

		let jwt = self
			.db
			.signup(Scope {
				namespace: &self.namespace,
				database: &self.database,
				scope: &self.scope,
				params: &signup,
			})
			.await?;

		trace!(message = "New user signed up", ?jwt);

		let new_user: Option<UserRecord> = self
			.db
			.query("SELECT * FROM type::table($table) WHERE email = $email")
			.bind(("email", &signup.email))
			.bind(("table", &self.users_table))
			.await?
			.take(0)?;

		match new_user {
			None => {
				let message = "Couldn't find user after signup";
				warn!(%message, ?signup);
				Err(AuthError::InternalInvariantBroken(message.into()))
			}
			Some(user) => {
				trace!(message = "Found new user id");
				Ok(user)
			}
		}
	}
}
