use crate::prelude::*;

pub(crate) async fn list_users<Config: DBAuthConfig, C: Connection>(
	config: &Config,
	db: &Surreal<C>,
) -> Result<Vec<UserRecord>, AuthError> {
	// let users: Vec<UserRecord> = db
	// 	.query("SELECT * FROM type::table($table)")
	// 	.bind(("table", config.users_table()))
	// 	.await?
	// 	.take(0)?;
	let users: Vec<UserRecord> = db.select(config.users_table()).await?;

	trace!(?users);

	Ok(users)
}
