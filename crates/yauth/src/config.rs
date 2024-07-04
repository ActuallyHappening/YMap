use crate::{prelude::*, types::UserRecord};
use surrealdb::opt::auth::Jwt;
use ysurreal::config::DBConnectRemoteConfig;

pub trait DBAuthConfig: DBConnectRemoteConfig {
	fn users_table(&self) -> String;

	fn users_scope(&self) -> String;

	/// Signs up, and switches to primary namespace and database.
	/// *Automatically signs in as well*
	fn sign_up<C: Connection>(
		&self,
		db: &Surreal<C>,
		credentials: &crate::signup::SignUp,
	) -> impl Future<Output = Result<(Jwt, crate::types::UserRecord), AuthError>> + Send + Sync
	where
		Self: Sized,
	{
		crate::signup::sign_up(self, db, credentials)
	}

	/// Signs into the scope assuming the user has already signed up, and switches to primary ns and db.
	fn sign_in<C: Connection>(
		&self,
		db: &Surreal<C>,
		credentials: &crate::signin::SignIn,
	) -> impl Future<Output = Result<(Jwt, crate::types::UserRecord), AuthError>> + Send + Sync
	where
		Self: Sized,
	{
		crate::signin::sign_in(self, db, credentials)
	}

	/// Calls [Surreal::invalidate].
	fn invalidate<C: Connection>(
		&self,
		db: &Surreal<C>,
	) -> impl Future<Output = Result<(), AuthError>> + Send + Sync
	where
		Self: Sized,
	{
		crate::signout::invalidate(self, db)
	}

	fn list_users<C: Connection>(
		&self,
		db: &Surreal<C>,
	) -> impl Future<Output = Result<Vec<UserRecord>, AuthError>> + Send + Sync
	where
		Self: Sized,
	{
		crate::signup::list_users(self, db)
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
