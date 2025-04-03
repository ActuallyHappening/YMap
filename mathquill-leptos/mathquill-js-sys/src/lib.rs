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

#[wasm_bindgen]
#[derive(Clone)]
pub struct Config {
  #[wasm_bindgen(js_name = "spaceBehavesLikeTab")]
  pub space_behaves_like_tab: bool,

  #[wasm_bindgen(getter_with_clone)]
  pub handlers: Handlers,
}

/// https://docs.mathquill.com/en/latest/Config/#handlers
#[wasm_bindgen]
#[derive(Clone)]
pub struct Handlers {
  #[wasm_bindgen(getter_with_clone)]
  pub edit: js_sys::Function,
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
    config: Option<Config>,
  ) -> StaticMath;

  pub type MathField;
  /// https://docs.mathquill.com/en/latest/Api_Methods/#mqstaticmathhtml_element
  #[wasm_bindgen(method)]
  pub fn MathField(
    this: &MathQuill,
    html_element: &web_sys::HtmlElement,
    config: Option<Config>,
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
