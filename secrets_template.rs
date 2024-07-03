pub trait SecretsTemplate: Sized {
	fn ssh_name() -> String;
	fn production_password() -> String;
}

/// This file is not checked into version control.
#[path = "./secrets.rs"]
#[cfg(not(feature = "production"))]
mod secrets;

#[cfg(not(feature = "production"))]
pub use secrets::Secrets;

#[cfg(feature = "production")]
pub struct Secrets;
