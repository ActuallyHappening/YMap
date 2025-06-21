#[path = "tracing.rs"]
pub mod app_tracing;
pub mod prelude;

pub mod hash {
	use crate::prelude::*;

	type Hash = ();

	pub async fn hash(path: impl AsRef<Utf8Path>) -> Result<Hash> {
		let path = path.as_ref();
		if path.is_file() {
			todo!()
		} else if path.is_dir() {
			todo!()
		} else {
			Err(eyre!("Path neither file nor directory").note(format!("Path: {}", path)))
		}
	}
}

pub mod vfs {
	use crate::prelude::*;
}
