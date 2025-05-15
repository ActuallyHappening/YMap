use mathquill_js::MathQuill;

fn main() {
	// getting this is usually the responsibility of
	// a web framework, e.g. leptos
	let element: web_sys::HtmlElement = todo!();
	let mq = MathQuill::get_global_interface();

	let field = mq.mount_static_field(&element);

	// this is often set in response to application demands
	field.set_latex(r"\text{Hey, this is cool!}");
}
