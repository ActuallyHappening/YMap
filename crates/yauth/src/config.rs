use crate::prelude::*;
use ysurreal::config::DBConnectRemoteConfig;

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
