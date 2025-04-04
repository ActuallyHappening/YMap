use crate::prelude::*;

pub fn MathQuillField() -> impl IntoView {
  let node_ref = NodeRef::new();
  node_ref.on_load(|el: web_sys::HtmlSpanElement| {
    let mathquill = mathquill_js::MathQuill::get_global_interface();
    let field = mathquill.mount_field(&el, mathquill_js::Config::default());

    let current = field.latex();
    info!(?current, "MathQuillField mounted");
  });
  view! {
      <span node_ref=node_ref />
  }
}
