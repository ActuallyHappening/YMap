use crate::cmds::*;
use crate::prelude::*;

#[derive(Debug)]
pub struct AuthConnection<'db, C: Connection, Config> {
	pub(crate) db: &'db Surreal<C>,
	pub(crate) config: &'db Config,
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
		credentials: &signup::SignUp,
	) -> Result<(Jwt, crate::types::UserRecord), AuthError>
	where
		Self: Sized,
	{
		signup::sign_up(self.config, self.db, credentials).await
	}

	/// Signs into the scope assuming the user has already signed up, and switches to primary ns and db.
	///
	/// Also waits for the database to be ready.
	pub async fn sign_in(
		&self,
		credentials: &signin::SignIn,
	) -> Result<(Jwt, crate::types::UserRecord), AuthError>
	where
		Self: Sized,
	{
		signin::sign_in(self.config, self.db, credentials).await
	}

	/// Calls [Surreal::invalidate].
	pub async fn invalidate(&self) -> Result<(), AuthError>
	where
		Self: Sized,
	{
		signout::invalidate(self.config, self.db).await
	}

	pub async fn session_info(&self) -> Result<session_info::SessionInfo, AuthError>
	where
		Self: Sized,
	{
		session_info::session_info(&self.config, self.db).await
	}

	/// Debugging use only
	pub async fn list_users(&self) -> Result<Vec<UserRecord>, AuthError>
	where
		Self: Sized,
	{
		list_users::list_users(self.config, self.db).await
	}
}
