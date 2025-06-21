#[path = "tracing.rs"]
pub mod app_tracing;
pub mod prelude;

pub mod hash {
	use alloy_primitives::{FixedBytes, keccak256};

	use crate::prelude::*;

	type Hash = FixedBytes<32>;

	pub async fn hash(path: impl AsRef<Utf8Path>) -> Result<Hash> {
		let path = path.as_ref();
		if path.is_file() {
			let data = ystd::fs::read(path).await?;
			let hash = keccak256(&data);
			Ok(hash)
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
