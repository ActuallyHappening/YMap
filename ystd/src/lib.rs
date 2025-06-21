pub mod prelude {
	pub use camino::{Utf8Path, Utf8PathBuf};
	pub(crate) use color_eyre::Report;
}

pub mod io {
	use std::fmt::Display;

	use crate::prelude::*;

	#[derive(Debug, thiserror::Error)]
	#[error("{report}")]
	pub struct Error {
		pub report: Report,
		#[source]
		pub io: std::io::Error,
	}

	pub type Result<T> = std::result::Result<T, Error>;
}

pub mod fs {
	use crate::{io, prelude::*};

	pub async fn create_dir_all(path: &Utf8Path) -> io::Result<()> {
		tokio::fs::create_dir_all(&path)
			.await
			.map_err(|io| io::Error {
				report: Report::new(io.clone()),
				io,
			})
	}
}
