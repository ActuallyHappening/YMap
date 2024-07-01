use crate::prelude::*;

/// A type directly wrapping an inner, validatable type.
///
/// This type must never safely be instantiated without validating the inner type.
/// Builds upon [garde].
trait ValidatedType: Sized + Validate {
	type Inner: Serialize + DeserializeOwned;

	/// See [ValidatedType::try_new] for a safe alternative.
	///
	/// # SAFETY
	/// Type must not be read or used without first validating it.
	unsafe fn new_unchecked(inner: Self::Inner) -> Self;

	/// This is safe because type type must be valid if it exists at all.
	///
	/// See [ValidatedType::new_unchecked] the only way to construct this type without validation.
	fn deref(&self) -> &Self::Inner;

	/// Safe constructor that validates the inner type before returning.
	fn try_new(inner: Self::Inner) -> Result<Self, ValidationError>
	where
		<Self as Validate>::Context: std::default::Default,
	{
		// SAFETY: This is going to be validated before returned
		let this = unsafe { Self::new_unchecked(inner) };
		this.validate()?;
		Ok(this)
	}
}

/// Wraps validation errors, so as to not expose internals of [garde]
#[derive(Debug, thiserror::Error)]
#[error("Error validating: {0}")]
pub struct ValidationError(#[from] garde::Report);

use garde::Validate;
use serde::de::DeserializeOwned;
pub use types::*;
mod types;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
	#[error("A validation error occurred: {0}")]
	ValidationError(#[from] ValidationError),

	#[error("Some internal invariant was broken: {0}")]
	InternalInvariantBroken(String),

	#[error("An error occurred with the database: {0}")]
	SurrealError(#[from] surrealdb::Error),
}

#[derive(Debug, Validate)]
pub struct AuthConnection<'db, C: Connection> {
	#[garde(skip)]
	pub db: &'db Surreal<C>,

	#[garde(length(min = 1))]
	pub namespace: String,

	#[garde(length(min = 1))]
	pub database: String,

	#[garde(length(min = 1))]
	pub users_table: String,

	#[garde(length(min = 1))]
	pub scope: String,
}

mod signup {
	use surrealdb::opt::auth::{Jwt, Scope};

	use super::ValidatedType;
	use crate::prelude::*;

	/// User facing signup data request
	#[derive(Debug, garde::Validate, Serialize)]
	pub struct Signup {
		#[garde(dive)]
		pub username: Username,

		#[garde(dive)]
		pub password: Password,

		#[garde(dive)]
		pub email: Email,
	}

	impl Signup {
		pub fn new(username: String, password: String, email: String) -> Result<Self, ValidationError> {
			Ok(Signup {
				username: Username::try_new(username)?,
				password: Password::try_new(password)?,
				email: Email::try_new(email)?,
			})
		}
	}

	impl<'db, C: Connection> AuthConnection<'db, C> {
		#[instrument(skip_all)]
		pub async fn signup(&self, signup: Signup) -> Result<UserRecord, AuthError> {
			let jwt = self
				.db
				.signup(Scope {
					namespace: &self.namespace,
					database: &self.database,
					scope: &self.scope,
					params: &signup,
				})
				.await?;

			trace!(message = "New user signed up", ?jwt);

			let new_user: Option<UserRecord> = self
				.db
				.query("SELECT * FROM type::table($table) WHERE email = $email")
				.bind(("email", &signup.email))
				.bind(("table", &self.users_table))
				.await?
				.take(0)?;

			match new_user {
				None => {
					let message = "Couldn't find user after signup";
					warn!(%message, ?signup);
					Err(AuthError::InternalInvariantBroken(message.into()))
				}
				Some(user) => {
					trace!(message = "Found new user id");
					Ok(user)
				}
			}
		}
	}
}
