#[cfg(not(feature = "production"))]
#[path = "../../../secrets_template.rs"]
mod secrets_template;

#[cfg(not(feature = "production"))]
pub use secrets_template::{Secrets, SecretsTemplate};
