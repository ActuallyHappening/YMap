#![warn(missing_debug_implementations, rust_2018_idioms)]

pub mod prelude {
	// deps re-exports
	pub(crate) use color_eyre::eyre::{Context, Report};
	pub(crate) use garde::Validate;
	pub(crate) use serde::{Deserialize, Serialize};
	pub(crate) use std::future::Future;
	pub(crate) use surrealdb::{Connection, Surreal};
	pub(crate) use tracing::*;

	// internal re-exports
	pub(crate) use crate::types::{ValidatedType, ValidationError};

	// public exports
	pub use crate::config::DBAuthConfig;
	pub use crate::error::AuthError;
	pub use ysurreal::prelude::*;
}
use crate::prelude::*;
pub mod signin;
pub mod signup;
pub mod types;

pub mod config {
	use crate::{prelude::*, types::UserRecord};
	use surrealdb::opt::auth::Jwt;
	use ysurreal::config::DBConnectRemoteConfig;

	pub trait DBAuthConfig: DBConnectRemoteConfig {
		fn users_table(&self) -> String;

		fn users_scope(&self) -> String;

		/// Signs up, and switches to primary namespace and database.
		/// *Automatically signs in as well*
		fn sign_up<C: Connection>(
			&self,
			db: &Surreal<C>,
			signup: &crate::signup::Signup,
		) -> impl Future<Output = Result<crate::types::UserRecord, AuthError>> + Send + Sync
		where
			Self: Sized,
		{
			crate::signup::sign_up(self, db, signup)
		}

		/// Signs into the scope assuming the user has already signed up, and switches to primary ns and db.
		fn sign_in<C: Connection>(
			&self,
			db: &Surreal<C>,
			signup: &crate::signin::SignIn,
		) -> impl Future<Output = Result<crate::types::UserRecord, AuthError>> + Send + Sync
		where
			Self: Sized,
		{
			crate::signin::sign_in(self, db, signup)
		}

		fn list_users<C: Connection>(
			&self,
			db: &Surreal<C>,
		) -> impl Future<Output = Result<Vec<UserRecord>, AuthError>> + Send + Sync
		where
			Self: Sized,
		{
			crate::signup::list_users(self, db)
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
}
pub mod error {
	use crate::prelude::*;

	#[derive(Debug, thiserror::Error)]
	pub enum AuthError {
		#[error("A validation error occurred: {0}")]
		ValidationError(#[from] ValidationError),

		#[error("Some internal invariant was broken: {0}")]
		InternalInvariantBroken(#[from] InternalInvariantBroken),

		#[error("An error occurred with the database: {0}")]
		SurrealError(#[from] surrealdb::Error),
	}

	#[derive(Debug, thiserror::Error)]
	pub enum InternalInvariantBroken {
		#[error(
			"User was signed in to the scope, but no corresponding record was found in the users table"
		)]
		UserSignedInButNoRecord,
		#[error(
			"User was signed up to the scope, but no corresponding record was found in the users table"
		)]
		UserSignedUpButNoRecord,
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
			"end_user".into()
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
