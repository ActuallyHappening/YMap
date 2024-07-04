pub mod prelude {
	pub(crate) use leptos::*;
	pub(crate) use tracing::*;
}
use crate::prelude::*;

#[path = "website/login.rs"]
mod login;

fn main() {
	use tracing_subscriber::prelude::*;
	use tracing_subscriber::{EnvFilter, Registry};
	console_error_panic_hook::set_once();

	Registry::default()
		.with(
			EnvFilter::try_from_default_env()
				.or_else(|_| EnvFilter::try_new("info,ymap=debug,ysurreal=debug,yauth=debug"))
				.unwrap(),
		)
		.with(tracing_wasm::WASMLayer::new(
			tracing_wasm::WASMLayerConfig::default(),
		))
		.init();

	info!("Logging is setup");

	mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
	view! {
		<div class="container mx-auto">
			<login::LoginForm />
		</div>
	}
}
