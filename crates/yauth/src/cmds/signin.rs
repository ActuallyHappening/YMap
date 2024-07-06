use surrealdb::opt::auth::{Jwt, Scope};

use crate::{
	error::InternalInvariantBroken,
	prelude::*,
	cmds::signup::SignUp,
	types::{Email, Password, UserRecord},
};

#[derive(clap::Args, Validate, Serialize, Debug, Clone)]
pub struct SignIn {
	#[arg(long)]
	#[garde(dive)]
	pub email: Email,

	#[arg(long)]
	#[garde(dive)]
	pub password: Password,
}

impl From<SignUp> for SignIn {
	fn from(value: SignUp) -> Self {
		SignIn {
			email: value.email,
			password: value.password,
		}
	}
}

impl From<&SignUp> for SignIn {
	fn from(value: &SignUp) -> Self {
		SignIn {
			email: value.email.clone(),
			password: value.password.clone(),
		}
	}
}

/// See [DBAuthConfig::sign_up] for documentation
pub(crate) async fn sign_in<Config: DBAuthConfig, C: Connection>(
	config: &Config,
	db: &Surreal<C>,
	signin: &SignIn,
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
		.signin(Scope {
			namespace,
			database,
			scope,
			params: &signin,
		})
		.await?;

	db.wait_for(surrealdb::opt::WaitFor::Database).await;

	trace!("User signed up and signed in successfully");

	let new_user: Option<UserRecord> = db
		.query("SELECT * FROM type::table($table) WHERE email = $email")
		.bind(("table", config.users_table()))
		.bind(("email", &signin.email))
		.await?
		.take(0)?;

	trace!(?new_user, "User record identified successfully");

	new_user
		.into_iter()
		.next()
		.map(|user| (jwt, user))
		.ok_or(AuthError::InternalInvariantBroken(
			InternalInvariantBroken::UserSignedInButNoRecord,
		))
}
