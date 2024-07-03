pub trait SecretsTemplate: Sized {
	fn ssh_name() -> String;
	fn production_password() -> String;
}

/// This file is not checked into version control.
#[path = "./secrets.rs"]
#[cfg(feature = "development")]
mod secrets;

#[cfg(feature = "development")]
pub use secrets::Secrets;
