pub trait SecretsTemplate: Sized {
	fn ssh_name() -> &'static str;
	fn production_password() -> &'static str;
}

/// This file is not checked into version control.
#[path = "./secrets.rs"]
mod secrets;

pub use secrets::Secrets;