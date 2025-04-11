#![allow(non_snake_case)]

pub use error::AppError;

pub mod prelude;
pub mod main {
  use crate::prelude::*;

  pub fn main() {
    info!("Hydrating ...");
    leptos::mount::mount_to_body(|| view! { <crate::app::App /> });
    info!("Finished hydration");
  }
}
pub mod app;
pub mod error;
