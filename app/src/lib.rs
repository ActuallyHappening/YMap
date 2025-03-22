#![allow(non_snake_case)]

use leptos_meta::MetaTags;
use prelude::*;

pub use app::App;

mod app;
mod brand;
mod db;
mod errors;
mod prelude;
mod rendering_state;
mod stripe;
mod utils;

#[cfg(feature = "ssr")]
pub mod server_state;

#[path = "components/components.rs"]
pub mod components;

#[cfg(feature = "hydrate")]
pub mod hydrate;

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />

        <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
        <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png" />
        <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png" />
        <link rel="manifest" href="/site.webmanifest" />

        <AutoReload options=options.clone() />
        <HydrationScripts options />
        <MetaTags />

        // <script src="https://kit.fontawesome.com/5ad518afe.js" crossorigin="anonymous" async></script>

        <link
          // href="https://fonts.googleapis.com/css?family=League Spartan"
          href="/static/fonts/League Spartan.css"
          rel="stylesheet"
        />
        <link
          // href="https://fonts.googleapis.com/css?family=Archivo Black"
          href="/static/fonts/Archivo Black.css"
          rel="stylesheet"
        />
      </head>
      <body>
        <App />
      </body>
    </html>
  }
}
