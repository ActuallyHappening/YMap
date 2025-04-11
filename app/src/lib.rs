#![allow(non_snake_case)]

pub mod main {
  pub fn main() {
    tracing::info!("Hydrating ...");
    leptos::mount::mount_to_body(|| leptos::view! { <crate::app::App /> });
    tracing::info!("Finished hydration");
  }
}
pub mod app;
