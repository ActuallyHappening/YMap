use wasm_bindgen::prelude::*;
use web_sys::js_sys;

/// API beginning
#[wasm_bindgen]
unsafe extern "C" {
  /// https://docs.mathquill.com/en/latest/Api_Methods/#api-methods
  pub type MathQuill;

  #[wasm_bindgen(js_namespace = MathQuill)]
  pub fn getInterface(version: u8) -> MathQuill;
}

/// Use [`Config::get_js_value`] to confirm options names
#[derive(Default)]
pub struct Config {
  pub space_behaves_like_tab: Option<bool>,
  pub handlers: Handlers,
}

impl Config {
  pub fn get_js_value(&self) -> JsValue {
    let obj = js_sys::Map::new();
    obj.set(
      &"spaceBehavesLikeTab".into(),
      &self.space_behaves_like_tab.into(),
    );
    obj.unchecked_into()
  }
}

/// https://docs.mathquill.com/en/latest/Config/#handlers
///
/// You will have to think about manual memory management:
/// https://rustwasm.github.io/wasm-bindgen/reference/passing-rust-closures-to-js.html#heap-allocated-closures
#[derive(Default)]
pub struct Handlers {
  pub edit: Option<Closure<dyn FnMut()>>,
}

impl Handlers {
  pub fn get_js_value(&self) -> JsValue {
    let obj = js_sys::Map::new();
    obj.set(
      &"edit".into(),
      &self
        .edit
        .as_ref()
        .map(|closure| closure.as_ref().clone())
        .into(),
    );
    obj.unchecked_into()
  }
}

/// Mount
#[wasm_bindgen]
unsafe extern "C" {
  pub type StaticMath;
  /// https://docs.mathquill.com/en/latest/Api_Methods/#mqstaticmathhtml_element
  #[wasm_bindgen(method)]
  pub fn StaticMath(
    this: &MathQuill,
    html_element: &web_sys::HtmlElement,
    config: JsValue,
  ) -> StaticMath;

  pub type MathField;
  /// https://docs.mathquill.com/en/latest/Api_Methods/#mqstaticmathhtml_element
  #[wasm_bindgen(method)]
  pub fn MathField(
    this: &MathQuill,
    html_element: &web_sys::HtmlElement,
    config: JsValue,
  ) -> MathField;
}

/// Syncing
#[wasm_bindgen]
unsafe extern "C" {
  /// https://docs.mathquill.com/en/latest/Api_Methods/#latex
  #[wasm_bindgen(method)]
  pub fn latex(this: &StaticMath) -> String;

  /// https://docs.mathquill.com/en/latest/Api_Methods/#latex
  #[wasm_bindgen(method)]
  pub fn latex(this: &MathField) -> String;

  /// https://docs.mathquill.com/en/latest/Api_Methods/#latexlatex_string
  #[wasm_bindgen(js_name = "latex", method)]
  pub fn set_latex(this: &StaticMath, latex: &str);

  /// https://docs.mathquill.com/en/latest/Api_Methods/#latexlatex_string
  #[wasm_bindgen(js_name = "latex", method)]
  pub fn set_latex(this: &MathField, latex: &str);

  // https://docs.mathquill.com/en/latest/Api_Methods/#editable-mathfield-methods
  // there are definitely more very interesting methods,
  // like simulating user writing, focussing e.t.c
  // open a PR to add these!
}
