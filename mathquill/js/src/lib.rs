#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/loading.md"))]
//!
//! ## API Structure
//! Read the docs on the [MathQuill](crate::MathQuill) struct, everything starts from there.
//!
//! ## Contrived example
//! From their docs:
//! ```js
//! var answerSpan = document.getElementById('answer');
//! var answerMathField = MQ.MathField(answerSpan, {
//!   handlers: {
//!     edit: function() {
//!       var enteredMath = answerMathField.latex(); // Get entered math in LaTeX format
//!       checkAnswer(enteredMath);
//!     }
//!   }
//! });
//! ```
//! This would translate to something like:
//! ```rust
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/field.rs"))]
//! ```
//!
//! If you don't want the field to be editable, you can use a static field
//! and manually set the latext content to be whatever you want, like this:
//! ```rust
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/static.rs"))]
//! ```
//!
//! See the examples directory for more information

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

use web_sys::wasm_bindgen::prelude::Closure;

/// The primary interface to the MathQuill library.
pub struct MathQuill(mathquill_js_sys::MathQuill);

impl MathQuill {
	/// JS errors if the MathQuill library is not loaded already.
	///
	/// Will reference a global stored in the mathquill library,
	/// so you can call this as many times as you want and assume
	/// each instance is the same as the last:
	/// `By default, MathQuill overwrites the global MathQuill variable when loaded`,
	/// copied from <https://docs.mathquill.com/en/latest/Api_Methods/#api-methods>
	pub fn get_global_interface() -> Self {
		Self(mathquill_js_sys::MathQuill::getInterface(2))
	}

	/// When config.handlers is dropped, the mounted field's callbacks will be invalidated
	pub fn mount_field(
		&self,
		html_element: &web_sys::HtmlElement,
		config: &Config<MathField>,
	) -> MathField {
		debug!(el = ?html_element, "Mounting a (mutable) field");
		MathField(self.0.MathField(html_element, config.0.get_js_value()))
	}

	pub fn mount_static_field(&self, html_element: &web_sys::HtmlElement) -> StaticMath {
		debug!(el = ?html_element, "Mounting a static field");
		StaticMath(self.0.StaticMath(html_element))
	}

	pub fn get_field(&self, el: &web_sys::HtmlElement) -> Option<MathField> {
		self.0.get_field(el).map(MathField)
	}

	pub fn get_static_field(&self, el: &web_sys::HtmlElement) -> Option<StaticMath> {
		self.0.get_static_field(el).map(StaticMath)
	}
}

/// Read docs on [mathquill_js_sys::Config] about manual memory management
pub struct Config<MathField>(mathquill_js_sys::Config<<MathField as IntoInner>::Inner>)
where
	MathField: IntoInner;

impl<T> Default for Config<T>
where
	T: IntoInner,
{
	fn default() -> Self {
		Self(mathquill_js_sys::Config::default())
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

/// Wrapper around [mathquill_js_sys::MathField]
pub struct MathField(mathquill_js_sys::MathField);

/// Used internally, exposed for correctness in case
/// you are also using [mathquill_js_sys]
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

	pub fn set_latex(&self, latex: &str) {
		self.0.set_latex(latex);
	}
}

/// Wrapper around [mathquill_js_sys::StaticMath]
pub struct StaticMath(mathquill_js_sys::StaticMath);

impl IntoInner for StaticMath {
	type Inner = mathquill_js_sys::StaticMath;
}

impl StaticMath {
	pub fn latex(&self) -> String {
		self.0.latex()
	}

	pub fn set_latex(&self, latex: &str) {
		self.0.set_latex(latex);
	}
}
