#![allow(non_snake_case)]

pub mod prelude;
pub mod main {
  use crate::prelude::*;

  pub fn main() -> color_eyre::Result<()> {
    info!("Hydrating ...");
    leptos::mount::mount_to_body(|| view! { <crate::app::App /> });
    info!("Finished hydration");

    Ok(())
  }
}
pub mod app;
