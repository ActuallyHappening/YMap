use leptos_meta::{Link, Script};

use crate::{assets::AssetsBasePath, prelude::*};

/// Mathquill requires JQuery (idk why)
#[component]
pub fn JQueryScript(assets_dir: AssetsBasePath) -> impl IntoView {
  let src = assets_dir
    .jquery_js()
    .expect("Couldn't get jquery_js")
    .to_string();
  view! {
      <Script src />
  }
}

#[component]
pub fn MathQuillScript(assets_dir: AssetsBasePath) -> impl IntoView {
  let src = assets_dir
    .mathquill_js()
    .expect("Couldn't get mathquill js")
    .to_string();
  view! {
      <Script src />
  }
}

#[component]
pub fn MathQuillCss(assets_dir: AssetsBasePath) -> impl IntoView {
  let href = assets_dir
    .mathquill_css()
    .expect("Couldn't get mathquill css")
    .to_string();
  view! {
      <Link rel="stylesheet" href />
  }
}

/// All required assets in one!
#[component]
pub fn MathQuillAssets(assets_dir: AssetsBasePath) -> impl IntoView {
  view! {
    <JQueryScript assets_dir=assets_dir.clone() />
    <MathQuillScript assets_dir=assets_dir.clone() />
    <MathQuillCss assets_dir=assets_dir.clone() />
  }
}

pub fn MathQuillField() -> impl IntoView {
  let node_ref = NodeRef::new();
  node_ref.on_load(|el: web_sys::HtmlSpanElement| {
    let mathquill = crate::js::MathQuill::get_global_interface();
    let field = mathquill.mount_field(&el, crate::js::Config::default());

    let current = field.latex();
    info!(?current, "MathQuillField mounted");
  });
  view! {
      <span node_ref=node_ref />
  }
}
