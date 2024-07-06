use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum SessionInfo {
	/// Not end user
	SignedOut,
	SignedIn {
		expiration: u64,
	}
}

pub(crate) async fn session_info<Config: DBAuthConfig, C: Connection>(
	config: &Config,
	db: &Surreal<C>,
) -> Result<SessionInfo, AuthError> {
	todo!()
}