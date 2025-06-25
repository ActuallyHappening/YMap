use alloy_primitives::FixedBytes;

use crate::prelude::*;

pub use alloy_primitives::utils::{Keccak256, keccak256};
pub type Hash = FixedBytes<32>;

pub async fn debug_hash_from_path(path: impl AsRef<Utf8Path>) -> Result<Hash> {
	let path = path.as_ref();
	if path.is_file().await {
		let data = ystd::fs::read(path).await?;
		let hash = keccak256(&data);
		Ok(hash)
	} else if path.is_dir().await {
		todo!()
	} else {
		Err(eyre!("Path neither file nor directory").note(format!("Path: {}", path)))
	}
}

pub trait MinimalHasher {
	fn write(&mut self, bytes: &[u8]);
	fn finish(&self) -> Hash;
}

impl MinimalHasher for Keccak256 {
	fn finish(&self) -> Hash {
		self.clone().finalize()
	}
	fn write(&mut self, bytes: &[u8]) {
		self.update(bytes);
	}
}

pub trait ForwardsCompatHash {
	/// Usually a URL path,
	/// e.g. b"docs.rs/yit-json/yit_json"
	/// e.g. b"github.com/ActuallyHappening/yit-json".
	///
	/// Make sure this is unique to your implementation to maintain backwards and forwards compatability
	fn prefix(&self) -> &'static [u8];
	/// **Must** include your prefix
	fn hash<H: MinimalHasher + ?Sized>(&self, hasher: &mut H);
}
