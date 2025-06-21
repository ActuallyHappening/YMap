//! cargo add time -F macros,formatting
//! 

use camino::Utf8Path;
use time::{UtcOffset, macros::format_description};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::{
	format::{self, JsonFields},
	time::OffsetTime,
};

#[allow(unused)]
pub struct Guard {
	guard: WorkerGuard,
}

pub const LOGS_DIR: &str = if cfg!(not(debug_assertions)) {
	"/home/ah/Desktop/logs"
} else {
	"/home/ah/Desktop/Salt-Discordbot/logs"
};
pub const PREFIX: &str = "rust-discordbot.json";

pub fn install_tracing(filter: &str) -> color_eyre::Result<Guard> {
	use tracing_error::ErrorLayer;
	use tracing_subscriber::prelude::*;
	use tracing_subscriber::{EnvFilter, fmt};

	// let offset = UtcOffset::current_local_offset().unwrap_or_else(|err| {
	// 	::tracing::warn!(message = "Couldn't find local time offset", ?err);
	// 	UtcOffset::UTC
	// });
	// let timer = OffsetTime::new(offset, format_description!("[hour]:[minute]:[second]"));

	if !camino::Utf8PathBuf::from(LOGS_DIR).is_dir() {
		color_eyre::eyre::bail!("Logs directory not found");
	}

	let (file, guard) = tracing_appender::non_blocking(tracing_appender::rolling::daily(
		LOGS_DIR,
		PREFIX,
	));
	let file_layer = fmt::layer()
		.with_ansi(false)
		.event_format(format::format().json())
		// https://github.com/tokio-rs/tracing/issues/1365#issuecomment-828845393
		.fmt_fields(JsonFields::new())
		.with_writer(file);

	let fmt_layer = fmt::layer().with_target(true);
	let filter_layer = EnvFilter::try_from_default_env()
		.or_else(|_| EnvFilter::try_new(filter))
		.unwrap();

	tracing_subscriber::registry()
		.with(filter_layer)
		.with(file_layer)
		.with(fmt_layer)
		.with(ErrorLayer::default())
		.init();

	color_eyre::install()?;

	Ok(Guard { guard })
}
