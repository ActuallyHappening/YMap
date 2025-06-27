use std::borrow::Cow;
use std::ops::Deref;

use bevy_reflect::{Reflect, reflect_trait};
use ystd::fs;

use crate::hash::MinimalHasher;
use crate::vfs::Key;
use crate::{YitContext, storage};
use crate::{hash::ForwardsCompatHash, prelude::*};

#[derive(Debug)]
pub struct File<S>
where
	S: Storage,
{
	pub name: Key,
	pub storage: <S as Storage>::Encoded,
}

/// The key innovation of YIT, that files are treated as
/// more than just plain text.
///
/// Implementors of this type are expected to contain
/// the data for a file or subunit of VCS controlled data
// #[reflect_trait]
pub trait Storage {
	type Encoded: EncodedStorage;

	fn state(&self) -> &impl YitContext;
	async fn decode(&self, data: Vec<u8>) -> color_eyre::Result<Self::Encoded>;
	async fn encode(&self, encoded: Self::Encoded) -> Vec<u8>;
}

pub trait EncodedStorage: core::fmt::Debug + ForwardsCompatHash + 'static {}

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
pub struct Stateful<'s, YitContext, T> {
	pub state: &'s YitContext,
	pub inner: T,
}

impl<'s, YitContext, T> Deref for Stateful<'s, YitContext, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

pub type BuiltinStorages<'s, C> = BuiltinStoragesDiscriminants<'s, C>;

pub enum BuiltinStoragesDiscriminants<'s, C> {
	PlainText(plaintext::PlainText<'s, C>),
}

#[derive(Debug)]
pub enum BuiltinEncoded {
	PlainText(plaintext::PlainTextEncoded),
}

impl BuiltinEncoded {
	fn plaintext(self) -> color_eyre::Result<plaintext::PlainTextEncoded> {
		let Self::PlainText(storage) = self else {
			bail!("don't call .plaintext on non-plaintext builtin storage")
		};
		return Ok(storage);
	}
}

impl ForwardsCompatHash for BuiltinEncoded {
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

impl EncodedStorage for BuiltinEncoded {}

impl<'s, C> Storage for BuiltinStorages<'s, C>
where
	C: YitContext,
{
	type Encoded = BuiltinEncoded;

	fn state(&self) -> &impl YitContext {
		match self {
			Self::PlainText(storage) => storage.state(),
		}
	}

	async fn encode(&self, encoded: Self::Encoded) -> Vec<u8> {
		match self {
			Self::PlainText(storage) => {
				storage
					.encode(
						encoded
							.plaintext()
							.expect("to encode plaintext into plaintext"),
					)
					.await
			}
		}
	}

	async fn decode(&self, data: Vec<u8>) -> Result<Self::Encoded> {
		match self {
			Self::PlainText(fmt) => fmt.decode(data).await.map(BuiltinEncoded::PlainText),
		}
	}
}

impl<S> File<S>
where
	S: Storage,
{
	pub async fn snapshot(storage: &S, path: impl AsRef<Utf8Path>) -> color_eyre::Result<File<S>> {
		let path = path.as_ref();
		path.assert_file().await?;
		let key = Key::from(
			path.file_name()
				.wrap_err(format!("yit::storage::File::snapshot path has no filename"))?
				.to_owned(),
		);
		let data = fs::read(path).await?;
		let storage = storage.decode(data).await?;
		Ok(File { name: key, storage })
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
		storage::{EncodedStorage, Stateful, Storage},
	};

	#[derive(Debug, Default)]
	pub struct PlainTextMarker;
	pub type PlainText<'s, YitContext> = Stateful<'s, YitContext, PlainTextMarker>;

	#[derive(Reflect, Debug, Clone)]
	pub struct PlainTextEncoded(String);

	impl ForwardsCompatHash for PlainTextEncoded {
		fn prefix(&self) -> &'static [u8] {
			b"https://docs.rs/yit/latest/yit/storage/plaintext"
		}

		fn hash<H: crate::hash::MinimalHasher + ?Sized>(&self, hasher: &mut H) {
			hasher.write(self.prefix());
			hasher.write(self.0.as_bytes())
		}
	}

	impl EncodedStorage for PlainTextEncoded {}

	impl<'s, C> Storage for PlainText<'s, C>
	where
		C: YitContext,
	{
		type Encoded = PlainTextEncoded;

		fn state(&self) -> &impl YitContext {
			self.state
		}

		async fn encode(&self, data: Self::Encoded) -> Vec<u8> {
			data.0.into_bytes()
		}

		async fn decode(&self, data: Vec<u8>) -> Result<Self::Encoded>
		where
			Self: Sized,
		{
			String::from_utf8(Vec::from(data))
				.wrap_err("Couldn't parse data strictly as UTF8")
				.map(PlainTextEncoded)
		}
	}
}
