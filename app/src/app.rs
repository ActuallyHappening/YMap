use crate::prelude::*;
use mathquill_leptos::components::*;

pub fn App() -> impl IntoView {
  leptos_meta::provide_meta_context();

  let latex = RwSignal::new(String::new());
  let on_edit = Callback::new(move |new_latex: String| latex.set(new_latex));

  view! {
    <h1> "YMap" </h1>
    <MathQuillField on_edit=on_edit />
    <p> { move || latex.get() } </p>
  }
}
