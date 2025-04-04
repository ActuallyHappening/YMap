#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

use web_sys::wasm_bindgen::{JsCast as _, prelude::Closure};

pub struct MathQuill(mathquill_js_sys::MathQuill);

pub struct Config(mathquill_js_sys::Config);

impl Default for Config {
  fn default() -> Self {
    let closure: Closure<dyn FnMut()> = Closure::new(|| info!("Default editted!"));
    let callback = Box::leak(Box::new(closure));
    Self(mathquill_js_sys::Config {
      space_behaves_like_tab: true,
      handlers: mathquill_js_sys::Handlers {
        edit: callback.as_ref().clone().unchecked_into(),
      },
    })
  }
}

impl MathQuill {
  /// JS errors if the MathQuill library is not loaded already
  pub fn get_global_interface() -> Self {
    Self(mathquill_js_sys::MathQuill::getInterface(2))
  }

  pub fn mount_field(&self, html_element: &web_sys::HtmlElement, config: Config) -> MathField {
    MathField(self.0.MathField(html_element, Some(config.0)))
  }

  // pub fn mount_static(&self, node_ref: &web_sys::HtmlEle)
}

pub struct MathField(mathquill_js_sys::MathField);

impl MathField {
  pub fn latex(&self) -> String {
    self.0.latex()
  }
}
