use crate::{error::ReportedError, io, prelude::*};

pub async fn current_dir() -> io::Result<Utf8PathBuf> {
	io::asyncify(|| {
		std::env::current_dir()
			.map_err(|io| {
				let io = Arc::new(io);
				io::Error {
					report: Report::new(io.clone()).wrap_err("ystd::env::current_dir()"),
					inner: Some(io),
				}
			})
			.and_then(|path| {
				Utf8PathBuf::try_from(path.clone())
					.wrap_reported_err(format!("ystd::env::current_dir -> {:?}", path))
					.map_err(ReportedError::erase_inner)
			})
	})
	.await
}
