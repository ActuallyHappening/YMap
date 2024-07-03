use surrealdb::opt::auth::Scope;

use crate::{
	error::InternalInvariantBroken,
	prelude::*,
	types::{Password, UserRecord},
};

#[derive(Serialize, Debug)]
pub struct SignIn {
	email: String,
	password: Password,
}

/// See [DBAuthConfig::sign_up] for documentation
pub(crate) async fn sign_in<Config: DBAuthConfig, C: Connection>(
	config: &Config,
	db: &Surreal<C>,
	signin: &SignIn,
) -> Result<crate::types::UserRecord, AuthError> {
	debug!("Signing user up");
	let namespace = config.primary_namespace();
	let namespace = namespace.as_str();
	let database = config.primary_database();
	let database = database.as_str();
	let scope = config.users_scope();
	let scope = scope.as_str();

	db.use_ns(namespace).use_db(database).await?;

	db.signin(Scope {
		namespace,
		database,
		scope,
		params: &signin,
	})
	.await?;

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
		.ok_or(AuthError::InternalInvariantBroken(
			InternalInvariantBroken::UserSignedInButNoRecord,
		))
}
