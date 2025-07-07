use std::fmt::Display;

use crate::{error::ReportedError, io, prelude::*};

pub(crate) async fn asyncify<F, T>(f: F) -> io::Result<T>
where
	F: FnOnce() -> io::Result<T> + Send + 'static,
	T: Send + 'static,
{
	match tokio::task::spawn_blocking(f).await {
		Ok(t) => t,
		Err(err) => Err(io::Error::empty(
			Report::new(err).wrap_err("ystd::io background async task failed"),
		)),
	}
}

pub type Error = ReportedError<std::io::Error>;

impl Error {
	pub fn with_io(mut self, io: Arc<std::io::Error>) -> Self {
		self.inner = Some(io);
		self
	}
}

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) trait MapIoError<T> {
	fn map_err_std_io<F>(self, f: F) -> core::result::Result<T, Error>
	where
		F: FnOnce(Arc<std::io::Error>) -> Report;
}

impl<T> MapIoError<T> for core::result::Result<T, std::io::Error> {
	fn map_err_std_io<F>(self, f: F) -> core::result::Result<T, Error>
	where
		F: FnOnce(Arc<std::io::Error>) -> Report,
	{
		self.map_err(|io| {
			let io = Arc::new(io);
			Error::empty(f(io.clone())).with_io(io)
		})
	}
}
