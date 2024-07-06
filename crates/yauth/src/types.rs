//! Useful types

use std::fmt::Display;

use crate::prelude::*;
use surrealdb::sql::Thing;

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
#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize)]
#[error("Error validating: {0}")]
pub struct ValidationError(#[from] garde::Report);

impl ValidationError {
	/// Represents the error that no value was passed.
	///
	/// Not strictly the same as the actual error produced by [garde::Validate],
	/// but it is manually constructable
	pub fn empty(path: impl Into<String>, message: impl Into<String>) -> Self {
		let mut report = garde::Report::new();
		report.append(
			garde::Path::new(path.into()),
			garde::Error::new(message.into()),
		);
		Self(report)
	}
}

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

		impl $ty {
			pub fn as_str(&self) -> &str {
				self.deref()
			}
		}

		impl std::str::FromStr for $ty {
			type Err = ValidationError;

			fn from_str(s: &str) -> Result<Self, Self::Err> {
				<$ty as ValidatedType>::try_new(s.to_string())
			}
		}

		impl TryFrom<String> for $ty {
			type Error = ValidationError;

			fn try_from(value: String) -> Result<Self, Self::Error> {
				<$ty as ValidatedType>::try_new(value)
			}
		}

		impl From<$ty> for String {
			fn from(value: $ty) -> Self {
				value.0
			}
		}
	};
	(many: $($ty:ty),*) => {
		$(impl_validation_only! { $ty })*
	};
}

impl_validation_only!(many: Username, Password, Email);

#[derive(garde::Validate, Serialize, Debug, Clone, PartialEq)]
#[serde(transparent)]
#[garde(transparent)]
#[repr(transparent)]
pub struct Username(#[garde(length(min = 2, max = 25))] String);

impl Display for Username {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl Username {
	pub fn testing_rand() -> Self {
		use std::str::FromStr;
		Username::from_str(&format!(
			"Random username: {}{}",
			rand::random::<char>(),
			rand::random::<char>()
		))
		.unwrap()
	}
}

/// Implements [`Debug`](std::fmt::Debug) without exposing the password
#[derive(garde::Validate, Serialize, Clone)]
#[serde(try_from = "String")]
#[serde(into = "String")]
// #[serde(transparent)]
#[garde(transparent)]
#[repr(transparent)]
pub struct Password(#[garde(length(min = 7))] String);

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

impl Password {
	pub fn testing_rand() -> Self {
		use std::str::FromStr;
		Self::from_str(&format!(
			"Random password: {}{}",
			rand::random::<char>(),
			rand::random::<char>()
		))
		.unwrap()
	}
}

/// Implements [`Debug`](std::fmt::Debug) without exposing the password
#[derive(Deserialize, Clone)]
// #[serde(try_from = "String")]
// #[serde(into = "String")]
#[serde(transparent)]
#[repr(transparent)]
pub struct HashedPassword(String);

impl std::fmt::Debug for HashedPassword {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("HashedPassword")
			.field(&"hash<********>")
			.finish()
	}
}

impl Display for HashedPassword {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "hash<********>")
	}
}

#[derive(garde::Validate, Serialize, Debug, Clone, PartialEq)]
#[serde(try_from = "String")]
#[serde(into = "String")]
// #[serde(transparent)]
#[garde(transparent)]
#[repr(transparent)]
pub struct Email(#[garde(email, length(min = 5))] String);

impl Display for Email {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl Email {
	pub fn testing_rand() -> Self {
		use std::str::FromStr;
		let chars_allowed = "absdefghijklmnopqrstuvwxyz0123456789";
		let random_char1 = chars_allowed
			.chars()
			.nth(rand::random::<usize>() % chars_allowed.len())
			.unwrap();
		let random_char2 = chars_allowed
			.chars()
			.nth(rand::random::<usize>() % chars_allowed.len())
			.unwrap();
		Self::from_str(&format!("random@email{}{}.com", random_char1, random_char2,)).unwrap()
	}
}

/// The ID of a user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserID(Thing);

impl std::fmt::Display for UserID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// ignores the db portion
		self.0.id.fmt(f)
	}
}

/// What is stored in the [AuthConnection::users_table] table
#[derive(Debug, Deserialize)]
pub struct UserRecord {
	username: Username,
	email: Email,
	password: HashedPassword,
	id: UserID,
}

impl UserRecord {
	pub fn username(&self) -> &Username {
		&self.username
	}

	pub fn password(&self) -> &HashedPassword {
		&self.password
	}

	pub fn email(&self) -> &Email {
		&self.email
	}

	pub fn id(&self) -> UserID {
		self.id.clone()
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use super::*;

	#[test]
	fn rejects_empty() {
		assert!(Username::from_str("").is_err());
		assert!(Password::from_str("").is_err());
		assert!(Email::from_str("").is_err());
	}

	#[test]
	fn rejects_one_character() {
		assert!(Username::from_str("a").is_err());
		assert!(Password::from_str("b").is_err());
		assert!(Email::from_str("c").is_err());
	}

	#[test]
	fn usernames_validate() {
		let a_good_username = String::from("My username");
		assert_eq!(
			serde_json::from_str::<Username>(&serde_json::to_string(&a_good_username).unwrap()).unwrap(),
			Username(a_good_username)
		);
		let a_bad_username = r#""a""#;
		assert!(serde_json::from_str::<Username>(a_bad_username).is_err());
	}

	#[test]
	fn emails_validate() {
		let a_good_email = String::from("ah@example.com");
		assert_eq!(
			serde_json::from_str::<Email>(&serde_json::to_string(&a_good_email).unwrap()).unwrap(),
			Email(a_good_email)
		);
		let a_bad_email = r#""a23587490""#;
		assert!(serde_json::from_str::<Email>(a_bad_email).is_err());
	}
}
