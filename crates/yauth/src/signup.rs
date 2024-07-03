use crate::prelude::*;
use crate::types::{Email, Password, Username};

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
