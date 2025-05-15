#[cfg(not(target_arch = "wasm32"))]
pub use not_wasm::*;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

#[cfg(not(target_arch = "wasm32"))]
mod not_wasm {
	use time::{UtcOffset, macros::format_description};
	use tracing_subscriber::fmt::time::OffsetTime;

	use crate::prelude::*;

	pub fn install_tracing(
		default_env_filter: impl std::borrow::Borrow<str>,
	) -> color_eyre::Result<()> {
		use tracing_error::ErrorLayer;
		use tracing_subscriber::prelude::*;
		use tracing_subscriber::{EnvFilter, fmt};

		let offset = UtcOffset::current_local_offset().unwrap_or_else(|err| {
			warn!(message = "Couldn't find local time offset", ?err);
			UtcOffset::UTC
		});
		let timer = OffsetTime::new(
			offset,
			format_description!("[hour]:[minute]:[second] [offset_hour]"),
		);

		let fmt_layer = fmt::layer().with_target(true).with_timer(timer);
		let filter_layer = EnvFilter::try_from_default_env()
			.or_else(|_| EnvFilter::try_new(default_env_filter.borrow()))
			.unwrap();

		tracing_subscriber::registry()
			.with(filter_layer)
			.with(fmt_layer)
			.with(ErrorLayer::default())
			.init();

		color_eyre::install()?;

		Ok(())
	}
}

#[cfg(target_arch = "wasm32")]
mod wasm {
	use tracing_subscriber::prelude::*;
	use tracing_subscriber::{EnvFilter, Registry};

	pub fn install_tracing(
		default_env_filter: impl std::borrow::Borrow<str>,
	) -> color_eyre::Result<()> {
		Registry::default()
			.with(
				EnvFilter::try_from_default_env()
					.or_else(|_| EnvFilter::try_new(default_env_filter.borrow()))
					.unwrap(),
			)
			.with(tracing_wasm::WASMLayer::new(
				tracing_wasm::WASMLayerConfig::default(),
			))
			.init();

		Ok(())
	}
}
