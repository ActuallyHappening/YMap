use mathquill_js::{Config, MathField, MathQuill};

fn check_answer(latex: String) {
  // Whatever you want here
}

fn main() {
  // getting this is usually the responsibility of
  // a web framework, e.g. leptos
  let element: web_sys::HtmlElement = todo!();
  let mq = MathQuill::get_global_interface();

  let mut config = Config::default();
  config.handlers().on_edit_field(|| {
    let field: Option<MathField> = MathQuill::get_global_interface().get_field(&element);
    let latex = field.unwrap().latex();
    check_answer(latex);
  });

  let _field = mq.mount_field(&element, &config);

  // dropping config invalidates closures,
  // read docs on Config
}
