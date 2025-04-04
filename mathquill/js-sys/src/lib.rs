use std::marker::PhantomData;

use builder::ObjectBuilder;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

use wasm_bindgen::prelude::*;

mod builder;

/// API beginning
#[wasm_bindgen]
unsafe extern "C" {
  /// https://docs.mathquill.com/en/latest/Api_Methods/#api-methods
  pub type MathQuill;

  #[wasm_bindgen(js_namespace = MathQuill)]
  pub fn getInterface(version: u8) -> MathQuill;
}

/// Use [`Config::get_js_value`] to confirm options names
pub struct Config<MathField> {
  pub space_behaves_like_tab: Option<bool>,
  pub handlers: Handlers<MathField>,
}

impl<T> Default for Config<T> {
  fn default() -> Self {
    Self {
      space_behaves_like_tab: None,
      handlers: Handlers::default(),
    }
  }
}

impl<T> std::fmt::Debug for Config<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Config")
      .field("space_behaves_like_tab", &self.space_behaves_like_tab)
      .field("handlers", &self.handlers)
      .finish()
  }
}

impl Config<MathField> {
  pub fn get_js_value(&self) -> JsValue {
    let mut obj = ObjectBuilder::new();
    obj.set("spaceBehavesLikeTab", &self.space_behaves_like_tab.into());
    obj.set("handlers", &self.handlers.get_js_value());
    obj.build()
  }
}

/// https://docs.mathquill.com/en/latest/Config/#handlers
///
/// You will have to think about manual memory management:
/// https://rustwasm.github.io/wasm-bindgen/reference/passing-rust-closures-to-js.html#heap-allocated-closures
pub struct Handlers<MathField> {
  pub edit: Option<Closure<dyn FnMut()>>,
  pub _phantom: PhantomData<MathField>,
}

impl<T> Default for Handlers<T> {
  fn default() -> Self {
    Self {
      edit: None,
      _phantom: PhantomData,
    }
  }
}

impl<T> std::fmt::Debug for Handlers<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Handlers")
      .field("edit", &self.edit)
      .finish()
  }
}

impl Handlers<MathField> {
  pub fn get_js_value(&self) -> JsValue {
    let mut obj = ObjectBuilder::new();
    obj.set(
      "edit",
      &self
        .edit
        .as_ref()
        .map(|closure| closure.as_ref().clone())
        .into(),
    );
    obj.build()
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
