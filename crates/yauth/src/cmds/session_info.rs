use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum SessionInfo {
	/// Not end user
	SignedOut,
	SignedIn {
		expiration: u128,
	}
}

#[derive(Debug, Deserialize)]
struct Exp {
	exp: Option<u128>,
}

impl From<Exp> for SessionInfo {
	fn from(exp: Exp) -> Self {
		match exp.exp {
			Some(exp) => SessionInfo::SignedIn { expiration: exp },
			None => SessionInfo::SignedOut,
		}
	}
}

pub(crate) async fn session_info<Config: DBAuthConfig, C: Connection>(
	_config: &Config,
	db: &Surreal<C>,
) -> Result<SessionInfo, AuthError> {
	let exp: Option<Exp> = db.query("SELECT exp FROM $session").await?.take(0)?;

	trace!(?exp, message = "Found session expiration");

	let session: Option<serde_json::Value> = db.query("SELECT * FROM $session").await?.take(0)?;

	debug!(?session, message = "Found session", remove_me = true);

	Ok(exp.unwrap().into())
}