pub mod prelude {
	pub(crate) use smart_default::SmartDefault;
	pub(crate) use surrealdb::{Connection, Surreal};
	pub(crate) use tracing::*;
}

mod connection {
	use crate::prelude::*;

	trait ValidationOnly: Sized {
		type Inner;

		unsafe fn new_unchecked(inner: Self::Inner) -> Self;

		fn try_new(inner: Self::Inner) -> Result<Self, ValidationError>;
	}

	pub use types::*;
	mod types {
		use super::{ValidationError, ValidationOnly};
		use crate::prelude::*;

		macro_rules! impl_validation_only {
			($ty:ty) => {
				impl ValidationOnly for $ty {
					type Inner = String;

					unsafe fn new_unchecked(inner: Self::Inner) -> Self {
						Self(inner)
					}

					fn try_new(inner: Self::Inner) -> Result<Self, ValidationError> {
						// SAFETY: This is going to be validated before returned
						let this = unsafe { Self::new_unchecked(inner) };
						use garde::Validate;
						this.validate()?;
						Ok(this)
					}
				}
			};
			(many: $($ty:ty),*) => {
				$(impl_validation_only! { $ty })*
			};
	}

		impl_validation_only!(many: Username, Password, Email);

		#[derive(Debug, garde::Validate)]
		#[garde(transparent)]
		pub struct Username(#[garde(length(min = 2, max = 25))] String);

		/// Implements [`Debug`](std::fmt::Debug) without exposing the password
		#[derive(garde::Validate)]
		#[garde(transparent)]
		pub struct Password(#[garde(length(min = 7, max = 50))] String);

		impl std::fmt::Debug for Password {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				f.debug_tuple("Password").field(&"********").finish()
			}
		}

		#[derive(Debug, garde::Validate)]
		#[garde(transparent)]
		pub struct Email(#[garde(email)] String);
	}

	#[derive(Debug, garde::Validate)]
	pub struct AuthConnection<'db, C: Connection> {
		#[garde(skip)]
		pub db: &'db Surreal<C>,

		#[garde(length(min = 1))]
		pub users_table: String,
	}

	#[derive(Debug, garde::Validate)]
	pub struct Signup {
		#[garde(dive)]
		username: Username,
		#[garde(dive)]
		password: Password,
		#[garde(dive)]
		email: Email,
	}

	/// Wraps validation errors, so as to not expose internals of [garde]
	#[derive(Debug, thiserror::Error)]
	#[error("Error validating: {0}")]
	pub struct ValidationError(#[from] garde::Report);

	impl Signup {
		pub fn new(username: String, password: String, email: String) -> Result<Self, ValidationError> {
			Ok(Signup {
				username: Username::try_new(username)?,
				password: Password::try_new(password)?,
				email: Email::try_new(email)?,
			})
		}
	}
}

mod types {}
