use crate::prelude::*;

pub fn MathQuillField() -> impl IntoView {
  let node_ref = NodeRef::new();
  let handlers_drop_handle = RwSignal::new_local(None);
  node_ref.on_load(move |el: web_sys::HtmlSpanElement| {
    let mathquill = mathquill_js::MathQuill::get_global_interface();

    let mut config = mathquill_js::Config::default();
    config.handlers().on_edit(|| info!("Field editted!"));

    let field = mathquill.mount_field(&el, &config);

    handlers_drop_handle.set(Some(config));

    let current = field.latex();
    info!(?current, "MathQuillField mounted");
  });
  view! {
      <span class="mathquill" node_ref=node_ref />
  }
}
