use std::borrow::Cow;

use bevy_reflect::reflect_trait;

use crate::YitRoot;
use crate::hash::MinimalHasher;
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
#[reflect_trait]
pub trait Storage: ObjectSafeHash {
	fn fmt_to_string(&self, root: &YitRoot) -> String;
}

pub trait ObjectSafeHash {
	/// Must be the same as [ForwardsCompatHash::prefix]
	fn prefix(&self) -> &'static [u8];
	/// Must be the same as [ForwardsCompatHash::hash]
	fn hash(&self, hasher: &mut dyn MinimalHasher);
}

impl<T> ObjectSafeHash for T
where
	T: ForwardsCompatHash,
{
	fn prefix(&self) -> &'static [u8] {
		ForwardsCompatHash::prefix(self)
	}

	fn hash(&self, hasher: &mut dyn MinimalHasher) {
		ForwardsCompatHash::hash(self, hasher);
	}
}

pub enum GenericStorage {
	PlainText(plaintext::PlainText),
}

impl File {
	pub async fn snapshot(root: &YitRoot, path: impl AsRef<Utf8Path>) -> color_eyre::Result<File> {
		todo!()
	}
}

pub mod plaintext {
	//! Using this defeats much of the purpose of using YIT,
	//! but can help teach you how it works
	use crate::{YitRoot, hash::ForwardsCompatHash, prelude::*, storage::Storage};

	#[derive(Debug, Clone)]
	pub struct PlainText(String);

	impl ForwardsCompatHash for PlainText {
		fn prefix(&self) -> &'static [u8] {
			b"https://docs.rs/yit/latest/yit/storage/plaintext"
		}

		fn hash<H: crate::hash::MinimalHasher + ?Sized>(&self, hasher: &mut H) {
			hasher.write(self.prefix());
			hasher.write(self.0.as_bytes())
		}
	}

	impl Storage for PlainText {
		fn fmt_to_string(&self, root: &YitRoot) -> String {
			self.0.clone()
		}
	}
}
