use crate::{prelude::*, types::UserRecord};
use surrealdb::opt::auth::Jwt;
use ysurreal::config::DBConnectRemoteConfig;

#[derive(Debug)]
pub struct AuthConnection<'db, C: Connection, Config> {
	db: &'db Surreal<C>,
	config: &'db Config,
}

impl<'db, C: Connection, Config> AuthConnection<'db, C, Config>
where
	Config: DBAuthConfig,
{
	/// Signs up, and switches to primary namespace and database.
	///
	/// This implicitly signs in as well.
	/// Also waits for the database to be ready.
	pub async fn sign_up(
		&self,
		credentials: &crate::signup::SignUp,
	) -> Result<(Jwt, crate::types::UserRecord), AuthError>
	where
		Self: Sized,
	{
		crate::signup::sign_up(self.config, self.db, credentials).await
	}

	/// Signs into the scope assuming the user has already signed up, and switches to primary ns and db.
	///
	/// Also waits for the database to be ready.
	pub async fn sign_in(
		&self,
		credentials: &crate::signin::SignIn,
	) -> Result<(Jwt, crate::types::UserRecord), AuthError>
	where
		Self: Sized,
	{
		crate::signin::sign_in(self.config, self.db, credentials).await
	}

	/// Calls [Surreal::invalidate].
	pub async fn invalidate(&self) -> Result<(), AuthError>
	where
		Self: Sized,
	{
		crate::signout::invalidate(self.config, self.db).await
	}

	pub async fn list_users(&self) -> Result<Vec<UserRecord>, AuthError>
	where
		Self: Sized,
	{
		crate::signup::list_users(self.config, self.db).await
	}
}

pub trait DBAuthConfig: DBConnectRemoteConfig {
	fn users_table(&self) -> String;

	fn users_scope(&self) -> String;

	/// Use this to utilize this configuration.
	fn control_db<'db, C: Connection>(&'db self, db: &'db Surreal<C>) -> AuthConnection<'db, C, Self>
	where
		Self: Sized,
	{
		AuthConnection { db, config: self }
	}
}

impl<C> DBAuthConfig for &C
where
	C: DBAuthConfig,
{
	fn users_table(&self) -> String {
		C::users_table(self)
	}

	fn users_scope(&self) -> String {
		C::users_scope(self)
	}
}
