use std::borrow::Cow;

use crate::vfs::Key;
use crate::{hash::ForwardsCompatHash, prelude::*};

pub struct File<S = GenericStorage> {
	pub name: Key,
	pub storage: S,
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
	use crate::{hash::ForwardsCompatHash, prelude::*, storage::Storage};

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

	impl Storage for PlainText {
		fn fmt(&self) -> String {
			self.0.clone()
		}
	}
}
