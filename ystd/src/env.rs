use crate::{io, prelude::*};

pub async fn current_dir() -> io::Result<Utf8PathBuf> {
	io::asyncify(|| {
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
	})
	.await
}
