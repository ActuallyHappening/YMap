#[path = "tracing.rs"]
pub mod app_tracing;
pub mod prelude;

pub use root::*;
mod root;

pub mod hash {
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
		fn finish(self) -> Hash;
	}

	impl MinimalHasher for Keccak256 {
		fn finish(self) -> Hash {
			Keccak256::finalize(self)
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
		fn hash<H: MinimalHasher>(&self, hasher: &mut H);
	}
}

pub mod storage {
	use std::borrow::Cow;

	use crate::{hash::ForwardsCompatHash, prelude::*};

	pub struct File<S = GenericStorage> {
		pub name: Cow<'static, str>,
		pub s: S,
	}

	/// The key innovation of YIT, that files are treated as
	/// more than just plain text.
	///
	/// Implementors of this type are expected to contain
	/// the data for a file or subunit of VCS controlled data
	pub trait Storage: ForwardsCompatHash {
		fn fmt(&self) -> String;
	}

	pub struct GenericStorage {}

	pub mod plaintext {
		//! Using this defeats much of the purpose of using YIT,
		//! but can help teach you how it works
		use crate::{hash::ForwardsCompatHash, prelude::*};

		#[derive(Debug, Clone)]
		pub struct PlainText(String);

		impl ForwardsCompatHash for PlainText {
			fn prefix(&self) -> &'static [u8] {
				b"https://docs.rs/yit/latest/yit/storage/plaintext"
			}

			fn hash<H: crate::hash::MinimalHasher>(&self, hasher: &mut H) {
				hasher.write(self.prefix());
				hasher.write(self.0.as_bytes())
			}
		}
	}
}
