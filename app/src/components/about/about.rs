use crate::{components::article_styles, prelude::*};
stylance::import_crate_style!(about_style, "src/components/about/about.module.scss");

pub fn About() -> impl IntoView {
  let inner_html = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../public/static/html/about.html"
  ));
  view! { <article class=article_styles::article inner_html=inner_html /> }
}
