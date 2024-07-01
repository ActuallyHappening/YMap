//! Useful types

use std::fmt::Display;

use crate::prelude::*;
use surrealdb::sql::{Id, Thing};

/// A type directly wrapping an inner, validatable type.
///
/// This type must never safely be instantiated without validating the inner type.
/// Builds upon [garde].
pub(crate) trait ValidatedType: Sized + Validate {
	type Inner: Serialize + serde::de::DeserializeOwned;

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

macro_rules! impl_validation_only {
	($ty:ty) => {
		impl ValidatedType for $ty {
			type Inner = String;

			unsafe fn new_unchecked(inner: Self::Inner) -> Self {
				Self(inner)
			}

			fn deref(&self) -> &Self::Inner {
				&self.0
			}
		}

		impl<'de> serde::Deserialize<'de> for $ty
		{
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: serde::Deserializer<'de>,
			{
				let inner = <$ty as ValidatedType>::Inner::deserialize(deserializer)?;

				// SAFETY: This is going to be validated before returned
				let this = unsafe { Self::new_unchecked(inner) };
				this.validate().map_err(serde::de::Error::custom)?;
				Ok(this)
			}
		}

		impl std::ops::Deref for $ty {
			type Target = str;

			fn deref(&self) -> &Self::Target {
				<Self as ValidatedType>::deref(self)
			}
		}

		impl std::str::FromStr for $ty {
			type Err = ValidationError;

			fn from_str(s: &str) -> Result<Self, Self::Err> {
				<$ty as ValidatedType>::try_new(s.to_string())
			}
		}
	};
	(many: $($ty:ty),*) => {
		$(impl_validation_only! { $ty })*
	};
}

impl_validation_only!(many: Username, Password, Email);

#[derive(garde::Validate, Serialize, Debug, Clone)]
#[garde(transparent)]
pub struct Username(#[garde(length(min = 2, max = 25))] String);

impl Display for Username {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

/// Implements [`Debug`](std::fmt::Debug) without exposing the password
#[derive(garde::Validate, Serialize, Clone)]
#[garde(transparent)]
pub struct Password(#[garde(length(min = 7, max = 50))] String);

impl std::fmt::Debug for Password {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("Password").field(&"********").finish()
	}
}

impl Display for Password {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "********")
	}
}

#[derive(garde::Validate, Serialize, Debug, Clone)]
#[garde(transparent)]
pub struct Email(#[garde(email)] String);

impl Display for Email {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

/// The ID of a user
#[derive(Debug, Deserialize)]
pub struct UserID(Id);

#[derive(Debug, Deserialize)]
pub struct UserRecord {
	username: Username,
	email: Email,
	thing: Thing,
}

impl UserRecord {
	pub fn username(&self) -> &Username {
		&self.username
	}

	pub fn email(&self) -> &Email {
		&self.email
	}

	pub fn id(&self) -> UserID {
		UserID(self.thing.id.clone())
	}
}
