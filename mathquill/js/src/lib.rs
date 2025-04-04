#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

use web_sys::wasm_bindgen::prelude::Closure;

pub struct MathQuill(mathquill_js_sys::MathQuill);

pub struct Config(mathquill_js_sys::Config);

impl Default for Config {
  fn default() -> Self {
    let mut cfg = mathquill_js_sys::Config::default();

    cfg.space_behaves_like_tab = Some(true);

    Self(cfg)
  }
}

pub struct HandlersMut<'config>(&'config mut mathquill_js_sys::Handlers);
impl Config {
  pub fn handlers(&mut self) -> HandlersMut<'_> {
    HandlersMut(&mut self.0.handlers)
  }
}
impl<'config> HandlersMut<'config> {
  pub fn on_edit(&mut self, callback: impl FnMut() + 'static) {
    self.0.edit = Some(Closure::new(callback));
  }
}

// impl Default for Config {
//   fn default() -> Self {
//     let edit: Closure<dyn FnMut()> = Closure::new(|| info!("Default editted!"));
//     Self(mathquill_js_sys::Config {
//       space_behaves_like_tab: Some(true),
//       handlers: Some(mathquill_js_sys::Handlers { edit }),
//     })
//   }
// }

impl MathQuill {
  /// JS errors if the MathQuill library is not loaded already
  pub fn get_global_interface() -> Self {
    Self(mathquill_js_sys::MathQuill::getInterface(2))
  }

  /// When config.handlers is dropped, the mounted field's callbacks will be invalidated
  pub fn mount_field(&self, html_element: &web_sys::HtmlElement, config: &Config) -> MathField {
    MathField(self.0.MathField(html_element, config.0.get_js_value()))
  }

  // pub fn mount_static(&self, node_ref: &web_sys::HtmlEle)
}

pub struct MathField(mathquill_js_sys::MathField);

impl MathField {
  pub fn latex(&self) -> String {
    self.0.latex()
  }
}
