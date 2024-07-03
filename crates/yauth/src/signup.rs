use crate::error::InternalInvariantBroken;
use crate::prelude::*;
use crate::types::{Email, Password, UserRecord, Username};
use surrealdb::opt::auth::{Jwt, Scope};

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

pub(crate) async fn sign_up<Config: DBAuthConfig, C: Connection>(
	config: &Config,
	db: &Surreal<C>,
	signup: &Signup,
) -> Result<(Jwt, crate::types::UserRecord), AuthError> {
	debug!("Signing user up");
	let jwt = db
		.signup(Scope {
			namespace: config.primary_namespace().as_str(),
			database: config.primary_database().as_str(),
			scope: config.users_scope().as_str(),
			// params: &signup,
			params: &(),
		})
		.await?;

	trace!("User signed up successfully");

	let new_user: Vec<serde_json::Value> = db
		.query("SELECT * FROM type::table($table)")
		.bind(("email", &signup.email))
		.bind(("table", config.users_table()))
		.await?
		.take(0)?;

	trace!(?new_user);

	todo!()

	// new_user
	// 	.into_iter()
	// 	.next()
	// 	.map(|u| (jwt, u))
	// 	.ok_or(AuthError::InternalInvariantBroken(
	// 		InternalInvariantBroken::UserSignedUpButNoRecord,
	// 	))
}
