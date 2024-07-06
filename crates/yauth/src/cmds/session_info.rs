use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
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

#[derive(Debug, PartialEq)]
pub enum SessionInfo {
	/// Not signed into any scope
	///
	/// Maybe still a root user?
	SignedOut,

	/// Signed into the expected user scope.
	///
	/// This the session is signed into any other scope, [`session_info`] will
	/// return an error instead of this variant.
	SignedIn,
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
struct Session {
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

const QUERY: &str = "SELECT exp FROM $session FETCH sd";

pub(crate) async fn session_info<Config: DBAuthConfig, C: Connection>(
	config: &Config,
	db: &Surreal<C>,
) -> Result<SessionInfo, AuthError> {
	/// Sometimes only an {exp: None} is returned, which means not signed in
	///
	/// idk the conditions for this, but i handle it with this struct
	#[derive(Deserialize)]
	struct NoSession {
		exp: Option<()>,
	}
	let no_session: Option<NoSession> = db.query(QUERY).await?.take(0)?;
	match no_session {
		Some(NoSession { exp: None }) => {
			debug!(
				message = "No session was found at all, only the exp passed",
				note = "IDK why this condition is every hit"
			);
			return Ok(SessionInfo::SignedOut);
		}
		_ => {}
	}

	let session: Option<Session> = db.query(QUERY).await?.take(0)?;

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
		return Ok(SessionInfo::SignedOut);
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
	Ok(SessionInfo::SignedIn)
}

#[cfg(test)]
mod tests {
	use ysurreal::{config::start_blank_memory_db, configs::TestingMem};

	use super::*;

	const INIT_SURQL: &str = "";

	#[test_log::test(tokio::test)]
	async fn db_no_session() -> Result<(), AuthError> {
		let conn_config = TestingMem::rand(INIT_SURQL.into());
		let db = start_blank_memory_db(&conn_config).unwrap().await?;
		// conn_config.init_query(&db).await?;
		conn_config.use_primary_ns_db(&db).await?;
		let auth_config = crate::configs::TestingAuthConfig::new(&conn_config);
		let auth_conn = auth_config.control_db(&db);

		let session_info = auth_conn.session_info().await?;

		assert_eq!(session_info, SessionInfo::SignedOut);

		Ok(())
	}
}
