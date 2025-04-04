#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

use web_sys::wasm_bindgen::prelude::Closure;

pub struct MathQuill(mathquill_js_sys::MathQuill);

impl MathQuill {
  /// JS errors if the MathQuill library is not loaded already
  pub fn get_global_interface() -> Self {
    Self(mathquill_js_sys::MathQuill::getInterface(2))
  }

  /// When config.handlers is dropped, the mounted field's callbacks will be invalidated
  pub fn mount_field(
    &self,
    html_element: &web_sys::HtmlElement,
    config: &Config<MathField>,
  ) -> MathField {
    MathField(self.0.MathField(html_element, config.0.get_js_value()))
  }

  pub fn get_field(el: &web_sys::HtmlElement) -> Option<MathField> {
    todo!()
  }

  // pub fn mount_static(&self, node_ref: &web_sys::HtmlEle)
}

pub struct Config<MathField>(mathquill_js_sys::Config<<MathField as IntoInner>::Inner>)
where
  MathField: IntoInner;

impl<T> Default for Config<T>
where
  T: IntoInner,
{
  fn default() -> Self {
    let mut cfg = mathquill_js_sys::Config::default();

    cfg.space_behaves_like_tab = Some(true);

    Self(cfg)
  }
}

pub struct HandlersMut<'config, MathField>(
  &'config mut mathquill_js_sys::Handlers<<MathField as IntoInner>::Inner>,
)
where
  MathField: IntoInner;
impl<T> Config<T>
where
  T: IntoInner,
{
  pub fn handlers(&mut self) -> HandlersMut<'_, T> {
    HandlersMut(&mut self.0.handlers)
  }
}
impl<'config> HandlersMut<'config, MathField> {
  pub fn on_edit_field(&mut self, callback: impl FnMut() + 'static) {
    self.0.edit = Some(Closure::new(callback));
  }
}

pub struct MathField(mathquill_js_sys::MathField);

pub trait IntoInner {
  type Inner;
}

impl IntoInner for MathField {
  type Inner = mathquill_js_sys::MathField;
}

impl MathField {
  pub fn latex(&self) -> String {
    self.0.latex()
  }
}
