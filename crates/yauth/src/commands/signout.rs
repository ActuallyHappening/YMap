use crate::prelude::*;

/// Takes config to satisfy type generics, though not actually needed
pub(crate) async fn invalidate<Config: DBAuthConfig, C: Connection>(
	_config: &Config,
	db: &Surreal<C>,
) -> Result<(), AuthError> {
	db.invalidate().await?;
	debug!("Signed out successfully");

	Ok(())
}
