pub mod prelude {
	pub use crate::path::{Path, PathBuf, Utf8Path, Utf8PathBuf, YPath, YPathBuf};
	pub(crate) use color_eyre::Report;
	pub(crate) use color_eyre::eyre::{WrapErr as _, bail, eyre};
	pub(crate) use std::sync::Arc;
	pub use tracing::{debug, error, info, trace, warn};
}

/// Wrapper types around [camino]
pub mod path {
	use crate::prelude::*;

	/// [camino::Utf8Path] newtype
	#[repr(transparent)]
	pub struct Utf8Path(pub camino::Utf8Path);
	pub type YPath = Utf8Path;
	pub type Path = YPath;

	impl Utf8Path {
		pub fn new(path: &(impl AsRef<str> + ?Sized)) -> &Self {
			let path = camino::Utf8Path::new(path);
			unsafe { &*(path as *const camino::Utf8Path as *const Utf8Path) }
		}

		pub fn as_str(&self) -> &str {
			self.0.as_str()
		}
	}

	impl std::ops::Deref for YPath {
		type Target = camino::Utf8Path;

		fn deref(&self) -> &Self::Target {
			&self.0
		}
	}

	impl AsRef<std::path::Path> for YPath {
		fn as_ref(&self) -> &std::path::Path {
			self.0.as_std_path()
		}
	}

	impl std::fmt::Display for YPath {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			self.0.fmt(f)
		}
	}

	/// [camino::Utf8PathBuf] newtype
	pub struct Utf8PathBuf(pub camino::Utf8PathBuf);
	pub type YPathBuf = Utf8PathBuf;
	pub type PathBuf = YPathBuf;

	impl std::ops::Deref for PathBuf {
		type Target = YPath;

		fn deref(&self) -> &Self::Target {
			Path::new(self.0.as_str())
		}
	}

	impl AsRef<YPath> for YPathBuf {
		fn as_ref(&self) -> &YPath {
			Path::new(self.0.as_str())
		}
	}

	impl From<camino::Utf8PathBuf> for PathBuf {
		fn from(path: camino::Utf8PathBuf) -> Self {
			Self(path)
		}
	}

	impl From<&YPath> for PathBuf {
		fn from(path: &YPath) -> Self {
			Self(path.0.into())
		}
	}

	impl TryFrom<std::path::PathBuf> for PathBuf {
		type Error = color_eyre::Report;

		fn try_from(value: std::path::PathBuf) -> Result<Self, Self::Error> {
			value.try_into().wrap_err(
				"ystd::path Failed to convert from `std::path::PathBuf` to `ystd::path::PathBuf`",
			)
		}
	}

	impl std::fmt::Display for Utf8PathBuf {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			self.0.fmt(f)
		}
	}
}

pub mod io {
	use std::fmt::Display;

	use crate::prelude::*;

	#[derive(Debug, thiserror::Error)]
	#[error("{report}")]
	#[non_exhaustive]
	pub struct Error {
		pub report: Report,
		#[source]
		pub io: Option<Arc<std::io::Error>>,
	}

	impl Error {
		pub fn new(report: Report) -> Self {
			Self { report, io: None }
		}

		pub fn with_io(mut self, io: Arc<std::io::Error>) -> Self {
			self.io = Some(io);
			self
		}
	}

	pub type Result<T> = std::result::Result<T, Error>;
}

pub mod fs {
	use crate::{io, prelude::*};

	pub(crate) async fn asyncify<F, T>(f: F) -> io::Result<T>
	where
		F: FnOnce() -> io::Result<T> + Send + 'static,
		T: Send + 'static,
	{
		match tokio::task::spawn_blocking(f).await {
			Ok(t) => t,
			Err(err) => Err(io::Error::new(
				Report::new(err).wrap_err("ystd::io background async task failed"),
			)),
		}
	}

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
		tokio::fs::read(path.as_ref()).await.map_err(|io| {
			let io = Arc::new(io);
			io::Error {
				report: Report::new(io.clone())
					.wrap_err(format!("ystd::fs::read({})", path.as_ref())),
				io: Some(io),
			}
		})
	}

	pub async fn canonicalize(path: impl AsRef<Utf8Path>) -> io::Result<Utf8PathBuf> {
		let path = path.as_ref().to_path_buf();
		asyncify(move || {
			path.canonicalize_utf8().map(PathBuf::from).map_err(|io| {
				let io = Arc::new(io);
				io::Error::new(
					Report::new(io.clone()).wrap_err(format!("ystd::fs::canonicalize({})", path)),
				)
				.with_io(io)
			})
		})
		.await
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
					report: err.wrap_err(format!("ystd::env::current_dir() -> {:?}", path)),
					io: None,
				})
			})
	}
}
