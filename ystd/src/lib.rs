pub mod prelude {
	pub use camino::{Utf8Path, Utf8PathBuf};
	pub(crate) use color_eyre::Report;
	pub(crate) use std::sync::Arc;
	pub use tracing::{debug, error, info, trace, warn};
}

pub mod io {
	use std::fmt::Display;

	use crate::prelude::*;

	#[derive(Debug, thiserror::Error)]
	#[error("{report}")]
	pub struct Error {
		pub report: Report,
		#[source]
		pub io: Option<Arc<std::io::Error>>,
	}

	pub type Result<T> = std::result::Result<T, Error>;
}

pub mod fs {
	use crate::{io, prelude::*};

	pub async fn create_dir_all(path: impl AsRef<Utf8Path>) -> io::Result<()> {
		tokio::fs::create_dir_all(path.as_ref())
			.await
			.map_err(|io| {
				let io = Arc::new(io);
				io::Error {
					report: Report::new(io.clone())
						.wrap_err(format!("ystd::fs::create_dir_all({})", path.as_ref())),
					io: Some(io),
				}
			})
	}
	
	pub async fn read(path: impl AsRef<Utf8Path>) -> io::Result<Vec<u8>> {
		tokio::fs::read(path.as_ref())
			.await
			.map_err(|io| {
				let io = Arc::new(io);
				io::Error {
					report: Report::new(io.clone())
						.wrap_err(format!("ystd::fs::read({})", path.as_ref())),
					io: Some(io),
				}
			})
	}
}

pub mod env {
	use crate::{io, prelude::*};

	pub fn current_dir() -> io::Result<Utf8PathBuf> {
		std::env::current_dir()
			.map_err(|io| {
				let io = Arc::new(io);
				io::Error {
					report: Report::new(io.clone()).wrap_err("ystd::env::current_dir()"),
					io: Some(io),
				}
			})
			.and_then(|path| {
				Utf8PathBuf::try_from(path.clone()).map_err(|err| io::Error {
					report: Report::new(err)
						.wrap_err(format!("ystd::env::current_dir() -> {:?}", path)),
					io: None,
				})
			})
	}
}
