use crate::prelude::*;
use mathquill_leptos::components::*;

pub fn App() -> impl IntoView {
  leptos_meta::provide_meta_context();

  let mathquill_assets = mathquill_leptos::assets::AssetsBasePath::new(
    url::Url::parse("http://localhost:8080").unwrap(),
    "mathquill-0.10.1",
  )
  .unwrap();

  view! {
    <h1> "YMap" </h1>
    <MathQuillAssets assets_dir=mathquill_assets />

    <MathQuillField />
  }
}
