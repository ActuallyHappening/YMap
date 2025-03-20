use leptos_meta::{Stylesheet, Title};

use crate::prelude::*;

pub fn App() -> impl IntoView {
  leptos_meta::provide_meta_context();

  view! {
      <Stylesheet id="leptos" href="/pkg/ymap.css" />

      <Title text="YMap" />

      <leptos_router::components::Router>
        <main>
            <h1>"Yay it works!"</h1>
        </main>
      </leptos_router::components::Router>
  }
}
