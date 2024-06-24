use tracing_subscriber::{fmt, prelude::*};
use tracing::*;

// "processing non `RedrawRequested` event after the main event loop: AboutToWait"

fn main() {
	let env_filter =
		tracing_subscriber::EnvFilter::from(r#"info,[{message="abc"}]=error"#);
	let fmt_layer = fmt::Layer::default();
	let subscriber = tracing_subscriber::Registry::default()
		.with(env_filter)
		.with(fmt_layer);
	subscriber.init();

	info!("Should come through");
	warn!(message = "abc", note = "Should NOT come through!");
}
