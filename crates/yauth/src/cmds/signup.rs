use crate::error::InternalInvariantBroken;
use crate::prelude::*;
use crate::types::{Email, Password, UserRecord, Username};
use surrealdb::opt::auth::{Jwt, Scope};

/// User facing signup data request
#[derive(garde::Validate, clap::Args, Serialize, Clone, Debug)]
#[group(skip)]
pub struct SignUp {
	#[arg(long)]
	#[garde(dive)]
	pub username: Username,

	#[arg(long)]
	#[garde(dive)]
	pub password: Password,

	#[arg(long)]
	#[garde(dive)]
	pub email: Email,
}

impl SignUp {
	pub fn new(username: String, password: String, email: String) -> Result<Self, ValidationError> {
		Ok(SignUp {
			username: Username::try_new(username)?,
			password: Password::try_new(password)?,
			email: Email::try_new(email)?,
		})
	}

	pub fn testing_rand() -> Self {
		SignUp {
			username: Username::testing_rand(),
			password: Password::testing_rand(),
			email: Email::testing_rand(),
		}
	}
}

/// See [DBAuthConfig::sign_up] for documentation
pub(crate) async fn sign_up<Config: DBAuthConfig, C: Connection>(
	config: &Config,
	db: &Surreal<C>,
	signup: &SignUp,
) -> Result<(Jwt, crate::types::UserRecord), AuthError> {
	debug!("Signing user up");
	let namespace = config.primary_namespace();
	let namespace = namespace.as_str();
	let database = config.primary_database();
	let database = database.as_str();
	let scope = config.users_scope();
	let scope = scope.as_str();

	db.use_ns(namespace).use_db(database).await?;

	let jwt = db
		.signup(Scope {
			namespace,
			database,
			scope,
			params: &signup,
		})
		.await?;

	db.wait_for(surrealdb::opt::WaitFor::Database).await;

	trace!("User signed up and signed in successfully");

	let new_user: Option<UserRecord> = db
		.query("SELECT * FROM type::table($table) WHERE email = $email")
		.bind(("table", config.users_table()))
		.bind(("email", &signup.email))
		.await?
		.take(0)?;

	new_user
		.into_iter()
		.next()
		.map(|user| (jwt, user))
		.ok_or(AuthError::InternalInvariantBroken(
			InternalInvariantBroken::UserSignedUpButNoRecord,
		))
}

#[cfg(test)]
mod test {
	use serde_json::json;
	use std::str::FromStr;

	use super::*;

	#[test]
	fn signup_serializes() {
		let signup = SignUp {
			username: Username::from_str("my username").unwrap(),
			password: Password::from_str("my password").unwrap(),
			email: Email::from_str("me@example.com").unwrap(),
		};
		let expected = json!({
			"username": "my username",
			"password": "my password",
			"email": "me@example.com",
		});
		assert_eq!(serde_json::to_value(signup).unwrap(), expected);
	}
}
