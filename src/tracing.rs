//! cargo add time -F macros,formatting
//!

use crate::prelude::*;
use time::{UtcOffset, macros::format_description};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::{
	format::{self, JsonFields},
	time::OffsetTime,
};
use ystd::{fs, prelude::*};

#[allow(unused)]
pub struct Guard {
	guard: WorkerGuard,
}

pub async fn logs_dir() -> Result<Utf8PathBuf> {
	if let Some(dir) = option_env!("RUST_LOG_DIR") {
		let dir = Utf8Path::new(dir);
		if !dir.is_dir().await {
			fs::create_dir_all(dir).await?;
		}
		Ok(dir.to_owned())
	} else {
		// todo: proper automatic project detection
		// search current dir + parent dirs for dir called .yit,
		// then return .yit/logs/
		let current_dir = ystd::env::current_dir().await?;
		let mut cwd_ancestors = current_dir.ancestors();
		while let Some(root_dir) = cwd_ancestors.next() {
			let yit_dir = root_dir.join(".yit");
			if yit_dir.is_dir().await {
				let dir = yit_dir.join("logs");
				ystd::fs::create_dir_all(&dir).await?;
				return Ok(dir);
			}
		}
		Err(
			color_eyre::eyre::eyre!("No .yit directory found to send logs to")
				.note(format!("CWD: {}", current_dir)),
		)
	}
}
/// TODO: use yit proj hash
pub const PREFIX: &str = "yit";

pub async fn install_tracing(filter: &str) -> color_eyre::Result<Guard> {
	use tracing_error::ErrorLayer;
	use tracing_subscriber::prelude::*;
	use tracing_subscriber::{EnvFilter, fmt};

	// let offset = UtcOffset::current_local_offset().unwrap_or_else(|err| {
	// 	::tracing::warn!(message = "Couldn't find local time offset", ?err);
	// 	UtcOffset::UTC
	// });
	// let timer = OffsetTime::new(offset, format_description!("[hour]:[minute]:[second]"));

	let logs_dir = logs_dir().await?;
	let (file, guard) =
		tracing_appender::non_blocking(tracing_appender::rolling::daily(logs_dir, PREFIX));
	let file_layer = fmt::Layer::default()
		.with_ansi(false)
		.event_format(format::format().json())
		// https://github.com/tokio-rs/tracing/issues/1365#issuecomment-828845393
		.fmt_fields(JsonFields::new())
		.with_writer(file);

	let fmt_layer = fmt::Layer::default().with_target(true);
	let filter_layer = EnvFilter::try_from_default_env()
		.or_else(|_| EnvFilter::try_new(filter))
		.unwrap();

	tracing_subscriber::Registry::default()
		.with(filter_layer)
		.with(file_layer)
		.with(fmt_layer)
		.with(ErrorLayer::default())
		.init();

	color_eyre::install()?;

	Ok(Guard { guard })
}
