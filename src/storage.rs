use std::borrow::Cow;

use bevy_reflect::{Reflect, reflect_trait};
use ystd::fs;

use crate::YitContext;
use crate::hash::MinimalHasher;
use crate::vfs::Key;
use crate::{hash::ForwardsCompatHash, prelude::*};

#[derive(Debug)]
pub struct File<S = BuiltinStorages> {
	pub name: Key,
	pub storage: S,
}

/// The key innovation of YIT, that files are treated as
/// more than just plain text.
///
/// Implementors of this type are expected to contain
/// the data for a file or subunit of VCS controlled data
// #[reflect_trait]
pub trait Storage: ForwardsCompatHash {
	async fn fmt_to_data(&self, state: &impl YitContext) -> Vec<u8>;
	async fn parse_from_data(state: &impl YitContext, data: &[u8]) -> Result<Self>
	where
		Self: Sized;
}

mod test {
	use bevy_reflect::{
		func::{ArgList, ReflectFn as _},
		prelude::*,
	};

	#[test]
	fn reflection() {
		#[derive(Reflect, Clone, PartialEq, Debug)]
		#[reflect(opaque)]
		struct Result(serde_json::Value);

		fn create(a: i32, b: i32) -> Result {
			Result(serde_json::json!({ "a": a, "b": b }))
		}

		let args = ArgList::new().with_owned(25_i32).with_owned(75_i32);

		let value = create.reflect_call(args).unwrap().unwrap_owned();
		assert_eq!(
			value.try_take::<Result>().unwrap(),
			Result(serde_json::json!({ "a": 25, "b": 75 }))
		);
	}
}

// pub trait ObjectSafeHash {
// 	/// Must be the same as [ForwardsCompatHash::prefix]
// 	fn prefix(&self) -> &'static [u8];
// 	/// Must be the same as [ForwardsCompatHash::hash]
// 	fn hash(&self, hasher: &mut dyn MinimalHasher);
// }

// impl<T> ObjectSafeHash for T
// where
// 	T: ForwardsCompatHash,
// {
// 	fn prefix(&self) -> &'static [u8] {
// 		ForwardsCompatHash::prefix(self)
// 	}

// 	fn hash(&self, hasher: &mut dyn MinimalHasher) {
// 		ForwardsCompatHash::hash(self, hasher);
// 	}
// }

#[derive(Debug)]
pub enum BuiltinStorages {
	PlainText(plaintext::PlainText),
}

impl ForwardsCompatHash for BuiltinStorages {
	/// A transparent wrapper
	fn prefix(&self) -> &'static [u8] {
		b""
	}

	fn hash<H: MinimalHasher + ?Sized>(&self, hasher: &mut H) {
		match self {
			Self::PlainText(fmt) => fmt.hash(hasher),
		}
	}
}

impl Storage for BuiltinStorages {
	async fn fmt_to_data(&self, state: &impl YitContext) -> Vec<u8> {
		match self {
			Self::PlainText(fmt) => fmt.fmt_to_data(state).await,
		}
	}

	async fn parse_from_data(state: &impl YitContext, data: &[u8]) -> Result<Self>
	where
		Self: Sized,
	{
		match self {
			Self::PlainText(fmt) => fmt
				.parse_from_data(state, data)
				.await
				.map(BuiltinStorages::PlainText),
		}
	}
}

impl<S> File<S> {
	pub async fn snapshot(
		state: &impl YitContext,
		path: impl AsRef<Utf8Path>,
	) -> color_eyre::Result<File<S>>
	where
		S: Storage,
	{
		let path = path.as_ref();
		path.assert_file().await?;
		let key = Key::from(
			path.file_name()
				.wrap_err(format!("yit::storage::File::snapshot path has no filename"))?
				.to_owned(),
		);
		let data = fs::read(path).await?;
		let storage = S::parse_from_data(state, &data).await?;
		Ok(File { name: key, storage })
	}
}

pub mod plaintext {
	//! Using this defeats much of the purpose of using YIT,
	//! but can help teach you how it works
	use bevy_reflect::Reflect;

	use crate::{YitContext, hash::ForwardsCompatHash, prelude::*, storage::Storage};

	#[derive(Reflect, Debug, Clone)]
	// #[reflect(Storage)]
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
		async fn fmt_to_data(&self, _state: &impl YitContext) -> Vec<u8> {
			self.0.clone().into_bytes()
		}

		async fn parse_from_data(&self, _state: &impl YitContext, data: &[u8]) -> Result<Self>
		where
			Self: Sized,
		{
			String::from_utf8(Vec::from(data))
				.wrap_err("Couldn't parse data strictly as UTF8")
				.map(Self)
		}
	}
}
