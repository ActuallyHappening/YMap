use crate::prelude::*;
use mathquill_leptos::components::*;

pub fn App() -> impl IntoView {
  leptos_meta::provide_meta_context();

  view! {
    <h1> "YMap" </h1>
    <MathQuillField />
  }
}
