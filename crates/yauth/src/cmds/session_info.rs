use crate::prelude::*;

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum SessionError {
	#[error("User is signed into the wrong database: expected {expected:?}, found {found:?}")]
	WrongDatabase { expected: String, found: String },

	#[error("Users is signed into the wrong namespace: expected {expected:?}, found {found:?}")]
	WrongNamespace { expected: String, found: String },

	#[error("User is signed into the wrong scope: expected {expected:?}, found {found:?}")]
	WrongScope { expected: String, found: String },

	// may become possible in surrealdb 2.0
	// #[error("User is signed into the wrong table: expected {expected:?}, found {found:?}")]
	// WrongUserTable { expected: String, found: String },
	/// This shouldn't happen, and doesn't indicate not signed in.
	#[error("No authentication session found at all! This means the $session meta-variable was empty, maybe didn't pass --auth?")]
	NoSessionFound,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SessionInfo {
	/// No session / no credentials
	SignedOutCompletely,

	/// Root credentials
	RootSignedIn,
	/// Scoped user
	UserSignedIn,
}

impl SessionInfo {
	pub fn user_signed_in(&self) -> bool {
		matches!(self, Self::UserSignedIn)
	}
}

/// There are more fields
///
/// Example:
/// ```text
/// session = Some(Object {
///
/// "db": String("production"),
/// "exp": Number(1720319053),
/// "id": Null,
/// "ip": String("180.216.98.251"),
/// "ns": String("production"),
/// "or": String("http://127.0.0.1:6969"),
/// "sc": String("end_user"),
/// "sd": Object {
///   "tb": String("user"),
///   "id": Object {"String": String("ncuhiz2d3ibhbxxiycc8")}
/// },
/// "tk": Object {
///   "DB": String("production"),
///   "ID": String("user:ncuhiz2d3ibhbxxiycc8"),
///   "NS": String("production"),
///   "SC": String("end_user"),
///   "exp": Number(1720319053),
///   "iat": Number(1720232653),
///   "iss": String("SurrealDB"),
///   "jti": String("4efa3bae-f673-4a61-9466-cabf60f326c5"),
///   "nbf": Number(1720232653)
/// }
///
/// });
/// ```
#[derive(Debug, Deserialize)]
struct SignedInSession {
	/// UNIX timestamp
	///
	/// Should be [`None`] if [`Session::scope_data`] is [`None`] as well.
	exp: Option<u128>,

	/// Should be primary_database
	#[serde(rename = "db")]
	database: String,

	/// Should be primary namespace
	#[serde(rename = "ns")]
	namespace: String,

	/// Should be end user scope from config
	#[serde(rename = "sc")]
	scope: String,

	/// Requires `FETCH sd` in query, or will return record link ID instead of actual [UserRecord] data
	///
	/// If this is [`None`], then the user is not signed into any scope
	#[serde(rename = "sd")]
	scope_data: Option<UserRecord>,
}

/// Sometimes only an {exp: None} is returned, which means not signed in at all
#[derive(Deserialize)]
struct SignedOutSession {
	exp: Option<u128>,
}

const QUERY: &str = "SELECT * FROM $session FETCH sd";

pub(crate) async fn session_info<Config: DBAuthConfig, C: Connection>(
	config: &Config,
	db: &Surreal<C>,
) -> Result<SessionInfo, AuthError> {
	// check if permissions to even read the session variable exist
	let check_access: Result<surrealdb::Response, _> = db.query("SELECT exp FROM $session").await;
	if let Err(surrealdb::Error::Api(surrealdb::error::Api::Query(err))) = &check_access {
		warn!(
			message = "Assuming signed out when actually can't access the users table",
			note = "This is expected behaviour if you are calling `session_info` expecting to be in a surreal session with absolutely no permissions",
			?check_access,
			error_message = ?err,
		);
		return Ok(SessionInfo::SignedOutCompletely);
	}

	// trace!(remove_me = true, "Querying for no-session info");
	// let no_session: Option<SignedOutSession> = db.query(QUERY).await?.take(0)?;
	// match no_session {
	// 	Some(SignedOutSession { exp }) => {
	// 		debug!(
	// 			message = "No session was found at all, only the exp passed",
	// 			note = "IDK why this condition is every hit",
	// 			note = "exp is the expiration of the session, `None` meaning not in a session?",
	// 			?exp,
	// 		);
	// 		return Ok(SessionInfo::SignedOutCompletely);
	// 	}
	// 	_ => {}
	// }

	trace!(remove_me = true, "Querying for session info");
	let session: Option<SignedInSession> = db.query(QUERY).await?.take(0)?;

	// todo: rename to session when IDE kicks in
	let Some(exp) = session else {
		return Err(SessionError::NoSessionFound.into());
	};

	if exp.scope_data.is_none() {
		if exp.exp.is_some() {
			warn!(
				message = "Internal inconsistency: No scope data, but the session has an expiration time?",
				note = "You can safely ignore this, as it is an internal detail of `yauth`s implementation"
			);
		}
		// no scope data is provided if using root sign in
		return Ok(SessionInfo::RootSignedIn);
	}

	if config.primary_database() != exp.database {
		return Err(
			SessionError::WrongDatabase {
				expected: config.primary_database(),
				found: exp.database,
			}
			.into(),
		);
	}

	if config.primary_namespace() != exp.namespace {
		return Err(
			SessionError::WrongNamespace {
				expected: config.primary_namespace(),
				found: exp.namespace,
			}
			.into(),
		);
	}

	// this check may be possible with surrealdb 2.0
	// if config.users_table() != exp.scope_data.table {
	// 	return Err(
	// 		SessionError::WrongUserTable {
	// 			expected: config.users_table(),
	// 			found: exp.scope_data.table,
	// 		}
	// 		.into(),
	// 	);
	// }

	if config.users_scope() != exp.scope {
		return Err(
			SessionError::WrongScope {
				expected: config.users_scope(),
				found: exp.scope,
			}
			.into(),
		);
	}

	// all checks pass, signed in yay!
	Ok(SessionInfo::UserSignedIn)
}

#[cfg(test)]
mod tests {
	use color_eyre::eyre::Report;
	use ysurreal::prelude::*;

	use super::*;

	#[test_log::test(tokio::test)]
	async fn db_no_session() -> Result<(), Report> {
		let conn_config = TestingConfig::rand(String::default());
		let db = start_testing_db(&conn_config).await?;
		conn_config.use_primary_ns_db(&db).await?;

		// don't sign in, don't initialize anything
		// conn_config.init_query(&db).await?;
		// conn_config.root_sign_in(&db).await?;
		let auth_config = crate::configs::TestingAuthConfig::new(&conn_config);
		let auth_conn = auth_config.control_db(&db);

		let session_info = auth_conn.session_info().await?;
		assert_eq!(session_info, SessionInfo::SignedOutCompletely);

		Ok(())
	}

	#[test_log::test(tokio::test)]
	async fn db_no_session_signed_out() -> Result<(), Report> {
		let conn_config = TestingConfig::rand(String::default());
		let db = start_testing_db(&conn_config).await?;
		conn_config.use_primary_ns_db(&db).await?;

		// conn_config.init_query(&db).await?;
		conn_config.root_sign_in(&db).await?;
		let auth_config = crate::configs::TestingAuthConfig::new(&conn_config);
		let auth_conn = auth_config.control_db(&db);

		// signs out first
		db.invalidate().await?;
		let session_info = auth_conn.session_info().await?;
		assert_eq!(session_info, SessionInfo::SignedOutCompletely);

		Ok(())
	}
}
