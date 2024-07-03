use crate::prelude::*;

pub(crate) async fn invalidate<Config: DBAuthConfig, C: Connection>(
	config: &Config,
	db: &Surreal<C>,
) -> Result<(), AuthError> {
	db.invalidate().await?;
	debug!("Signed out successfully");

	Ok(())
}