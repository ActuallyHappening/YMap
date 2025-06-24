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

pub mod path_matching;

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
