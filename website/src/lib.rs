pub mod prelude {
	// dep exports
	pub(crate) use cfg_if::cfg_if;
	pub(crate) use http::status::StatusCode;
	pub(crate) use leptonic::prelude::*;
	pub(crate) use leptos::*;
	pub(crate) use serde::{Deserialize, Serialize};
	pub(crate) use tracing::*;

	// surrealdb exports
	pub(crate) use surrealdb::engine::any::Any;
	pub(crate) use surrealdb::opt::auth::Jwt;
	pub(crate) use surrealdb::Surreal;

	// linked project exports
	pub(crate) use yauth::prelude::*;

	// internal re-exports
	pub(crate) use crate::error::AppError;
}

pub mod app;
pub mod error;
pub mod pages;
pub mod state;

pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;

/// Called only on client side in a browser, mounts to <body>
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	use crate::app::*;
	use crate::prelude::*;
	use leptos::*;

	// console_error_panic_hook::set_once();
	// tracing_wasm::set_as_global_default_with_config(
	// 		tracing_wasm::WASMLayerConfigBuilder::default()
	// 				.set_max_level(tracing::Level::DEBUG)
	// 				.build(),
	// );

	// leptos::mount_to_body(App);

	use tracing_subscriber::prelude::*;
	use tracing_subscriber::{EnvFilter, Registry};

	console_error_panic_hook::set_once();

	Registry::default()
		.with(
			EnvFilter::try_from_default_env()
				.or_else(|_| {
					EnvFilter::try_new("info,ymap-website=trace,ymap=debug,ysurreal=debug,yauth=debug")
				})
				.unwrap(),
		)
		.with(tracing_wasm::WASMLayer::new(
			tracing_wasm::WASMLayerConfig::default(),
		))
		.init();

	info!("Logging is setup");
	trace!("Using a client side rendering function, make sure to deploy using SSR for HTTPS support");

	// tracing_wasm::set_as_global_default_with_config(
	//     tracing_wasm::WASMLayerConfigBuilder::default()
	//         .set_max_level(tracing::Level::DEBUG)
	//         .build(),
	// );

	mount_to_body(|| {
		view! { <App/> }
	});
}
