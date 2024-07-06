use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
	#[error("User is signed into the wrong database: expected {expected:?}, found {found:?}")]
	WrongDatabase {
		expected: String,
		found: String,
	},

	#[error("Users is signed into the wrong namespace: expected {expected:?}, found {found:?}")]
	WrongNamespace{
		expected: String,
		found: String,
	},

	#[error("User is signed into the wrong scope: expected {expected:?}, found {found:?}")]
	WrongScope{
		expected: String,
		found: String,
	},

	#[error("User is signed into the wrong table: expected {expected:?}, found {found:?}")]
	WrongUserTable{
		expected: String,
		found: String,
	},

	#[error("No authentication session found at all! This means the $session meta-variable was empty, maybe didn't pass --auth?")]
	NoSessionFound,
}

#[derive(Debug, Clone)]
pub enum SessionInfo {
	/// Not end user
	SignedOut,
	SignedIn(Session)
}

#[derive(Debug, Deserialize, Clone)]
pub struct Session {
	exp: u128,

	/// Should be primary_database
	#[serde(rename = "db")]
	database: String,
	
	#[serde(rename = "ns")]
	namespace: String,

	#[serde(rename = "sc")]
	scope: String,

	#[serde(rename = "sd")]
	scope_data: ScopeData,
}

/// There are more fields
/// 
/// Example:
/// ```text
/// session = Some(Object {"db": String("production"), "exp": Number(1720319053), "id": Null, "ip": String("180.216.98.251"), "ns": String("production"), "or": String("http://127.0.0.1:6969"), "sc": String("end_user"), "sd": Object {"tb": String("user"), "id": Object {"String": String("ncuhiz2d3ibhbxxiycc8")}}, "tk": Object {"DB": String("production"), "ID": String("user:ncuhiz2d3ibhbxxiycc8"), "NS": String("production"), "SC": String("end_user"), "exp": Number(1720319053), "iat": Number(1720232653), "iss": String("SurrealDB"), "jti": String("4efa3bae-f673-4a61-9466-cabf60f326c5"), "nbf": Number(1720232653)}});
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct ScopeData {
	#[serde(rename = "db")]
	table: String,
}

pub(crate) async fn session_info<Config: DBAuthConfig, C: Connection>(
	_config: &Config,
	db: &Surreal<C>,
) -> Result<SessionInfo, AuthError> {
	let exp: Option<Session> = db.query("SELECT exp FROM $session").await?.take(0)?;

	let Some(exp) = exp else {
		return Err(SessionError::NoSessionFound.into())
	};



	todo!()
}