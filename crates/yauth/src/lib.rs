#![warn(missing_debug_implementations, rust_2018_idioms)]

pub mod prelude {
	// deps re-exports
	pub(crate) use garde::Validate;
	pub(crate) use serde::{Deserialize, Serialize};
	pub(crate) use std::future::Future;
	pub(crate) use surrealdb::{Connection, Surreal};
	pub(crate) use tracing::*;
	pub(crate) use color_eyre::eyre::{Context, Report};

	// internal re-exports
	pub(crate) use crate::types::{ValidatedType, ValidationError};

	// public exports
	pub use crate::error::AuthError;
	pub use ysurreal::prelude::*;
	pub use crate::config::DBAuthConfig;
}
use crate::prelude::*;

pub mod config {
	use surrealdb::opt::auth::{Jwt, Scope};
	use ysurreal::config::DBConnectRemoteConfig;

	use crate::{prelude::*, types::UserRecord};

	pub trait DBAuthConfig: DBConnectRemoteConfig {
		fn users_table(&self) -> String;

		fn users_scope(&self) -> String;

		/// Auto provided method to actually sign up
		#[must_use = "Futures do nothing unless you `.await` or poll them"]
		fn sign_up<C: Connection>(
			&self,
			db: &Surreal<C>,
			signup: &crate::signup::Signup,
		) -> impl Future<Output = Result<(Jwt, crate::types::UserRecord), AuthError>> + Send + Sync {
			async move {
				debug!("Signing user up");
				let jwt = db
					.signup(Scope {
						namespace: self.primary_namespace().as_str(),
						database: self.primary_database().as_str(),
						scope: self.users_scope().as_str(),
						params: &signup,
					})
					.await?;

				trace!("User signed up successfully");

				let new_user: Option<UserRecord> = db
					.query("SELECT * FROM type::table($table) WHERE email = $email")
					.bind(("email", &signup.email))
					.bind(("table", self.users_table()))
					.await?
					.take(0)?;

				new_user.map(|u| (jwt, u)).ok_or(AuthError::InternalInvariantBroken(String::from("User was signed up and signed into scope successfully, but could not be found in the database as expected")))
			}
		}
	}

	impl<C> DBAuthConfig for &C where C: DBAuthConfig {
		fn users_table(&self) -> String {
			C::users_table(self)
		}

		fn users_scope(&self) -> String {
			C::users_scope(self)
		}
	}
}

pub mod signup;
pub mod types;
pub mod error {
	use crate::prelude::*;

	#[derive(Debug, thiserror::Error)]
	pub enum AuthError {
		#[error("A validation error occurred: {0}")]
		ValidationError(#[from] ValidationError),

		#[error("Some internal invariant was broken: {0}")]
		InternalInvariantBroken(String),

		#[error("An error occurred with the database: {0}")]
		SurrealError(#[from] surrealdb::Error),
	}
}

pub mod configs {
	use ysurreal::config::DBConnectRemoteConfig;

	use crate::{config::DBAuthConfig, prelude::*};

	#[derive(Debug)]
	pub struct TestingAuthConfig<InnerConfig> {
		testing_connection: InnerConfig,
	}

	impl<InnerConfig> TestingAuthConfig<InnerConfig> {
		pub fn new(testing_connection: InnerConfig) -> Self {
			TestingAuthConfig { testing_connection }
		}
	}

	impl<InnerConfig> DBAuthConfig for TestingAuthConfig<InnerConfig>
	where
		InnerConfig: DBConnectRemoteConfig,
	{
		fn users_table(&self) -> String {
			"user".into()
		}

		fn users_scope(&self) -> String {
			"user".into()
		}
	}

	impl<InnerConfig> DBConnectRemoteConfig for TestingAuthConfig<InnerConfig>
	where
		InnerConfig: DBConnectRemoteConfig,
	{
		fn primary_database(&self) -> String {
			self.testing_connection.primary_database()
		}

		fn primary_namespace(&self) -> String {
			self.testing_connection.primary_namespace()
		}

		fn connect_host(&self) -> String {
			self.testing_connection.connect_host()
		}

		fn connect_port(&self) -> u16 {
			self.testing_connection.connect_port()
		}
	}
}