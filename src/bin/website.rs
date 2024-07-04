use crate::prelude::*;
use leptos::*;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::prelude::*;
use tracing::*;

fn main() {
	console_error_panic_hook::set_once();

	Registry::default()
    .with(EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info,ymap=debug,ysurreal=debug,yauth=debug")).unwrap())
		.with(tracing_wasm::WASMLayer::new(
			tracing_wasm::WASMLayerConfig::default(),
		))
		.init();

	info!("Logging is setup");

	mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
