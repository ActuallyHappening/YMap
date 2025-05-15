use app::prelude::*;

fn main() -> color_eyre::Result<()> {
	console_error_panic_hook::set_once();
	utils::tracing::install_tracing(
		"debug,app=trace,mathquill-leptos=trace,mathquill-js=trace,mathquill-js-sys=trace,latex-parser=trace,cas=trace",
	)?;

	info!("Initialized logging on front-end");

	app::main::main()
}
