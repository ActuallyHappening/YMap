use db::auth;
use leptos::{config::LeptosOptions, prelude::use_context};

/// Provided as `leptos::provide_context` in the server.
///
/// Cheap to [`Clone`]
#[derive(Clone)]
pub struct ServerAxumState {
  pub leptos_options: LeptosOptions,
  pub db: db::Db<auth::Root>,
  pub stripe: payments::server::ServerStripeController,
}

impl ServerAxumState {
  pub fn from_context() -> ServerAxumState {
    use_context().expect("Must have provided ServerAxumState already")
  }
}

impl axum::extract::FromRef<ServerAxumState> for LeptosOptions {
  fn from_ref(input: &ServerAxumState) -> Self {
    input.leptos_options.clone()
  }
}
