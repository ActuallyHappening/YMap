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
	tokio::fs::read(path.as_ref()).await.map_err(|io| {
		let io = Arc::new(io);
		io::Error {
			report: Report::new(io.clone()).wrap_err(format!("ystd::fs::read({})", path.as_ref())),
			io: Some(io),
		}
	})
}

pub async fn canonicalize_utf8(path: impl AsRef<Utf8Path>) -> io::Result<Utf8PathBuf> {
	let path = path.as_ref().to_owned();
	let path = crate::io::asyncify(move || {
		path.0.canonicalize_utf8().map(Utf8PathBuf).map_err(|io| {
			let io = Arc::new(io);
			io::Error::new(
				Report::new(io.clone()).wrap_err(format!("ystd::fs::canonicalize({})", path)),
			)
			.with_io(io)
		})
	})
	.await?;
	Ok(path)
}

pub async fn canonicalize(path: impl AsRef<Utf8Path>) -> io::Result<Utf8PathBuf> {
	canonicalize_utf8(path).await
}
