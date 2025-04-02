pub mod prelude;
pub mod main {
  use crate::prelude::*;

  pub fn main() -> color_eyre::Result<()> {
    info!("Hydrating ...");
    leptos::mount::mount_to_body(|| view! { <h1> "Yay!" </h1> });
    info!("Finished hydration");

    Ok(())
  }
}
