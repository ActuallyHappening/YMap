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
	/// Primary method of signing a new user up
	///
	/// Waits for the database to connect before continuing.
	/// See <https://docs.rs/surrealdb/latest/surrealdb/opt/enum.WaitFor.html#variant.Database>
	/// and <https://docs.rs/surrealdb/latest/surrealdb/struct.Surreal.html#method.wait_for>
	#[instrument(skip_all)]
	pub async fn signup(&self, signup: Signup) -> Result<UserRecord, AuthError> {
		let username = signup.username.as_str();
		let email = signup.email.as_str();
		// let password = signup.password.as_str();
		let users_table = self.auth_instance.users_table.as_str();
		let database = self.auth_instance.database.as_str();
		let namespace = self.auth_instance.namespace.as_str();
		let scope = self.auth_instance.scope.as_str();
		debug!(
			message = "Signing up new user",
			?username,
			?email,
			?users_table,
			?scope,
			?database,
			?namespace,
			note = "Errors are not reported on the same verbosity as this log",
			note = "Also, waiting for Database to connect"
		);

		let jwt = self
			.db
			.signup(Scope {
				namespace,
				database,
				scope,
				params: &signup,
			})
			.await?;

		trace!(message = "New user signed up", ?jwt);

		// why does this not work?
		let new_user: Option<UserRecord> = self
			.db
			.query("SELECT * FROM type::table($table) WHERE email = $email")
			.bind(("email", email))
			.bind(("table", users_table))
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

// #[cfg(test)]
// mod tests {
// 	use ysurreal::testing::TestingDBConnection;

// 	use crate::prelude::*;

// 	#[tokio::test]
// 	async fn testing_blank_signup_works() {
// 		let conn_options = TestingDBConnection::from_env();
// 		let db = conn_options.connect_ws().await.unwrap();
// 		let auth_con = AuthConnection {
// 			db: &db,
// 		};
// 	}
// }
