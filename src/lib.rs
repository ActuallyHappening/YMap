#[path = "tracing.rs"]
pub mod app_tracing;
pub mod prelude;

pub use root::*;
mod root {
	use crate::prelude::*;

	pub struct YitRoot {
		/// Canonicalized
		dir: Utf8PathBuf,
	}

	impl YitRoot {
		pub async fn new(dir: impl AsRef<Utf8Path>) -> Result<Self> {
			let dir = dir.as_ref().to_path_buf();
			let dir = dir.canonicalize().await?;
			dir.assert_dir().await?;
			Ok(Self { dir })
		}

		pub fn dir(&self) -> &Utf8Path {
			&self.dir
		}

		/// Makes sure the path provided is within this Yit root directory
		pub async fn resolve_local_path(
			&self,
			path: impl AsRef<Utf8Path>,
		) -> color_eyre::Result<Utf8PathBuf> {
			let path = path.as_ref().canonicalize().await?;
			for ancestor in path.ancestors() {
				if *ancestor == *self.dir {
					return Ok(path);
				}
			}
			bail!(
				"Path {} isn't within the yit project root of {}",
				path,
				self.dir
			);
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[tokio::test]
		async fn resolve_local_path() -> color_eyre::Result<()> {
			let root = YitRoot::new(env!("CARGO_MANIFEST_DIR")).await?;
			let path = root.dir().join("src").join("lib.rs");
			eyre_assert_eq!(root.resolve_local_path(&path).await?, path);
			Ok(())
		}
	}
}

pub mod hash {
	use alloy_primitives::{FixedBytes, keccak256};

	use crate::prelude::*;

	type Hash = FixedBytes<32>;

	pub async fn hash(path: impl AsRef<Utf8Path>) -> Result<Hash> {
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
}

pub mod vfs {
	use std::hash::Hash;

	use alloy_primitives::Keccak256;

	use crate::prelude::*;

	#[derive(Hash, Clone, Debug, PartialEq, Eq)]
	pub enum Vfs {
		File(Vec<u8>),
		Dir(Vec<Vfs>),
	}

	#[derive(Clone, Debug, PartialEq, Eq)]
	pub enum Test {
		A(u32),
		B(u32),
	}

	impl Hash for Test {
		fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
			match self {
				Test::A(a) => {
					state.write(b"https://example.com");
					a.hash(state);
				}
				Test::B(b) => b.hash(state),
			}
		}
	}

	#[test]
	fn hashing_derive() {
		let a = Test::A(1);
		let b = Test::B(1);

		let mut hasher1 = Keccak256::new();
		// a.hash(&mut hasher1);
		// let hasher2 = Keccak256::new();
	}
}
