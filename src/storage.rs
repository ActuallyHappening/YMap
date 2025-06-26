use std::borrow::Cow;

use bevy_reflect::{Reflect, reflect_trait};

use crate::YitContext;
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
	fn fmt_to_data(&self) -> Vec<u8>;

	// fn consume_data(&mut self, input: &[u8]);
}

mod test {
	use bevy_reflect::{
		func::{ArgList, ReflectFn as _},
		prelude::*,
	};

	#[test]
	fn reflection() {
		fn add(a: i32, b: i32) -> i32 {
			a + b
		}

		let args = ArgList::new().with_owned(25_i32).with_owned(75_i32);

		let value = add.reflect_call(args).unwrap().unwrap_owned();
		assert_eq!(value.try_take::<i32>().unwrap(), 100);
	}
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

pub struct GenericStorage {
	/// NB: Must implement Storage at least
	inner: Box<dyn Reflect>,
}

impl GenericStorage {
	pub fn new<S: Storage + Reflect>(storage: S) -> GenericStorage {
		Self::new_unchecked(storage)
	}

	/// storage must Reflect implement [Storage] in the
	/// yit root's type registry
	pub fn new_unchecked<S>(storage: S) -> GenericStorage
	where
		S: Reflect,
	{
		Self {
			inner: Box::new(storage),
		}
	}

	pub fn try_new<S: Reflect>(root: &impl YitContext, storage: S) -> Option<GenericStorage> {
		let registration = root.registry().get(core::any::TypeId::of::<S>())?;
		if registration.contains::<ReflectStorage>() {
			Some(GenericStorage::new_unchecked(storage))
		} else {
			None
		}
	}
}

impl<S> File<S> {
	pub async fn snapshot(
		root: &impl YitContext,
		path: impl AsRef<Utf8Path>,
	) -> color_eyre::Result<File<S>> {
		todo!()
	}
}

pub mod plaintext {
	//! Using this defeats much of the purpose of using YIT,
	//! but can help teach you how it works
	use bevy_reflect::Reflect;

	use crate::{
		YitContext,
		hash::ForwardsCompatHash,
		prelude::*,
		storage::{ReflectStorage, Storage},
	};

	#[derive(Reflect, Debug, Clone)]
	#[reflect(Storage)]
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
		fn fmt_to_data(&self) -> Vec<u8> {
			self.0.clone().into_bytes()
		}
	}
}
