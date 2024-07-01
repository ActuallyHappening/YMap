use garde::Validate;
use surrealdb::sql::{Id, Thing};

use super::ValidatedType;
use crate::prelude::*;

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
	};
	(many: $($ty:ty),*) => {
		$(impl_validation_only! { $ty })*
	};
}

impl_validation_only!(many: Username, Password, Email);

#[derive(Debug, garde::Validate, Serialize)]
#[garde(transparent)]
pub struct Username(#[garde(length(min = 2, max = 25))] String);

/// Implements [`Debug`](std::fmt::Debug) without exposing the password
#[derive(garde::Validate, Serialize)]
#[garde(transparent)]
pub struct Password(#[garde(length(min = 7, max = 50))] String);

impl std::fmt::Debug for Password {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("Password").field(&"********").finish()
	}
}

#[derive(Debug, garde::Validate, Serialize)]
#[garde(transparent)]
pub struct Email(#[garde(email)] String);

/// The ID of a user
#[derive(Debug, Deserialize)]
pub struct UserID(Id);

#[derive(Debug, Deserialize)]
pub struct UserRecord {
	pub name: Username,
	pub email: Email,
	thing: Thing,
}

impl UserRecord {
	pub fn id(&self) -> UserID {
		UserID(self.thing.id.clone())
	}
}