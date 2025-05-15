use wasm_bindgen::JsValue;
use web_sys::js_sys::{self, Reflect};

pub(crate) struct ObjectBuilder(js_sys::Object);

impl ObjectBuilder {
	pub fn new() -> Self {
		Self(js_sys::Object::new())
	}

	pub fn set(&mut self, key: &str, value: &JsValue) -> &mut Self {
		Reflect::set(&self.0, &JsValue::from(key), value).unwrap();
		self
	}

	pub fn build(self) -> JsValue {
		JsValue::from(self.0)
	}
}
