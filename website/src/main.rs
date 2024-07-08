pub mod prelude {
	pub(crate) use crate::app::{AppError, AppState};
	pub(crate) use leptonic::prelude::*;
	pub(crate) use leptos::*;
	pub(crate) use leptos_router::*;
	pub(crate) use serde::{Deserialize, Serialize};
	pub(crate) use surrealdb::engine::any::Any;
	pub(crate) use surrealdb::Surreal;
	pub(crate) use thiserror::Error;
	pub(crate) use tracing::*;
	pub(crate) use yauth::prelude::*;
}

mod app;
mod error_template;
mod pages;

use crate::app::*;
use crate::prelude::*;

fn main() {
	console_error_panic_hook::set_once();
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
	trace!("Traces comming through");

	// tracing_wasm::set_as_global_default_with_config(
	//     tracing_wasm::WASMLayerConfigBuilder::default()
	//         .set_max_level(tracing::Level::DEBUG)
	//         .build(),
	// );

	mount_to_body(|| {
		view! { <App/> }
	});
}
