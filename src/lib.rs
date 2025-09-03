pub fn init_debug_tools(filter: &str) -> color_eyre::Result<()> {
	#[cfg(not(target_arch = "wasm32"))]
	{
		use tracing_error::ErrorLayer;
		use tracing_subscriber::fmt::{self};
		use tracing_subscriber::layer::SubscriberExt as _;
		use tracing_subscriber::util::SubscriberInitExt as _;
		use tracing_subscriber::EnvFilter;
		let fmt_layer = fmt::Layer::default().with_target(true);
		let filter_layer = EnvFilter::try_from_default_env()
			.or_else(|_| EnvFilter::try_new(filter))
			.unwrap();

		tracing_subscriber::Registry::default()
			.with(filter_layer)
			.with(fmt_layer)
			.with(ErrorLayer::default())
			.init();

		color_eyre::install()?;
		Ok(())
	}

	#[cfg(target_arch = "wasm32")]
	{
		use tracing_subscriber::prelude::*;
		console_error_panic_hook::set_once();
		tracing_subscriber::registry::Registry::default()
			.with(tracing_wasm::WASMLayer::new(
				tracing_wasm::WASMLayerConfig::default(),
			))
			.init();
		Ok(())
	}
}
