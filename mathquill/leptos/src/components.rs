use std::ops::Deref as _;

use crate::prelude::*;

#[component]
pub fn MathQuillField(#[prop(into)] on_edit: Callback<String>) -> impl IntoView {
  let node_ref = NodeRef::<leptos::html::Span>::new();
  let handlers_drop_handle = RwSignal::new_local(None);

  node_ref.on_load(move |el: web_sys::HtmlSpanElement| {
    let mathquill = mathquill_js::MathQuill::get_global_interface();

    let mut config = mathquill_js::Config::default();
    config.handlers().on_edit_field(move || {
      let mathquill = mathquill_js::MathQuill::get_global_interface();
      let field = mathquill
        .get_field(&node_ref.get_untracked().unwrap().into())
        .expect("Not to be unmounted at this point, since mathquill is calling us");
      let latex = field.latex();
      on_edit.run(latex);
    });

    let field = mathquill.mount_field(&el, &config);

    handlers_drop_handle.set(Some(config));

    let current = field.latex();
    info!(?current, "MathQuillField mounted");
  });

  view! {
      <span class="mathquill" node_ref=node_ref />
  }
}

#[component]
pub fn MathQuillStatic(#[prop(into)] latex: Signal<String>) -> impl IntoView {
  let node_ref = NodeRef::<leptos::html::Span>::new();

  node_ref.on_load(move |el: web_sys::HtmlSpanElement| {
    let mathquill = mathquill_js::MathQuill::get_global_interface();
    let field = mathquill.mount_static_field(&el);
    // sets initial value
    field.set_latex(&latex.read_untracked());

    // set latex on signal change
    Effect::new(move || {
      let node_ref = node_ref.read();
      let Some(node_ref) = node_ref.deref() else {
        warn!("Latex signal changed on non-existant el");
        return;
      };
      let mathquill = mathquill_js::MathQuill::get_global_interface();
      let Some(field) = mathquill.get_static_field(node_ref) else {
        warn!(?node_ref, "MathQuillStatic field not found");
        return;
      };
      let new = latex.read();
      field.set_latex(&new);
    });
  });

  view! {
      <span class="mathquill" node_ref=node_ref />
  }
}
