use std::borrow::Cow;
use std::ops::Deref;

use bevy_reflect::{Reflect, reflect_trait};
use ystd::fs;

use crate::hash::MinimalHasher;
use crate::vfs::Key;
use crate::{YitContext, storage};
use crate::{hash::ForwardsCompatHash, prelude::*};

pub struct File<'c, C, S>
where
	S: Storage<'c, C>,
	C: YitContext,
{
	pub name: Key,
	pub storage: <S as Storage<'c, C>>::Encoded,
}

impl<'c, C, S> core::fmt::Debug for File<'c, C, S>
where
	S: Storage<'c, C>,
	C: YitContext,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("File")
			.field("name", &self.name)
			.field("storage", &self.storage)
			.finish()
	}
}

/// The key innovation of YIT, that files are treated as
/// more than just plain text.
///
/// Implementors of this type are expected to contain
/// the data for a file or subunit of VCS controlled data
// #[reflect_trait]
pub trait Storage<'c, C>
where
	C: YitContext,
{
	type Encoded: EncodedStorage;

	fn state(&self) -> &'c C;
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

pub enum BuiltinStorages<'c, C> {
	PlainText(plaintext::PlainTextStorage<'c, C>),
	Toml(toml::TomlStorage<'c, C>),
}

impl<'c, C> BuiltinStorages<'c, C>
where
	C: YitContext,
{
	pub async fn default_by_file_extension(
		context: &C,
		path: impl AsRef<Utf8Path>,
	) -> color_eyre::Result<Self> {
		let path = path.as_ref();
		let path = context.resolve_local_path(path).await?;
		let extension = path.extension()?;
		todo!()
	}
}

#[derive(Debug)]
pub enum BuiltinEncoded {
	PlainText(plaintext::PlainTextEncoded),
	Toml(toml::TomlEncoded),
}

impl BuiltinEncoded {
	fn plaintext(self) -> color_eyre::Result<plaintext::PlainTextEncoded> {
		let Self::PlainText(storage) = self else {
			bail!("don't call .plaintext on non-plaintext builtin storage")
		};
		return Ok(storage);
	}

	fn toml(self) -> color_eyre::Result<toml::TomlEncoded> {
		let Self::Toml(storage) = self else {
			bail!("don't call .toml on non-toml builtin storage")
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
			Self::Toml(fmt) => fmt.hash(hasher),
		}
	}
}

impl EncodedStorage for BuiltinEncoded {}

impl<'c, C> Storage<'c, C> for BuiltinStorages<'c, C>
where
	C: YitContext,
{
	type Encoded = BuiltinEncoded;

	fn state(&self) -> &'c C {
		match self {
			Self::PlainText(storage) => storage.state(),
			Self::Toml(storage) => storage.state(),
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
			Self::Toml(storage) => {
				storage
					.encode(encoded.toml().expect("to encode toml into toml"))
					.await
			}
		}
	}

	async fn decode(&self, data: Vec<u8>) -> Result<Self::Encoded> {
		match self {
			Self::PlainText(fmt) => fmt.decode(data).await.map(BuiltinEncoded::PlainText),
			Self::Toml(fmt) => fmt.decode(data).await.map(BuiltinEncoded::Toml),
		}
	}
}

impl<'c, C, S> File<'c, C, S>
where
	S: Storage<'c, C>,
	C: YitContext,
{
	pub async fn snapshot(storage: &S, path: impl AsRef<Utf8Path>) -> color_eyre::Result<Self> {
		let path = path.as_ref();
		path.assert_file().await?;

		let key = Key::from(
			path.file_name()
				.wrap_err(format!("yit::storage::File::snapshot path has no filename"))?
				.to_owned(),
		);
		let data = fs::read(path).await?;
		let storage = storage
			.decode(data)
			.await
			.wrap_err(format!("Couldn't decode raw data at path {}", path))?;
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
	#[non_exhaustive]
	pub struct PlainTextMarker;
	pub type PlainTextStorage<'s, YitContext> = Stateful<'s, YitContext, PlainTextMarker>;

	impl<'s, C> PlainTextStorage<'s, C> {
		pub fn new(state: &'s C) -> Self {
			Stateful {
				state,
				inner: PlainTextMarker,
			}
		}
	}

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

	impl<'c, C> Storage<'c, C> for PlainTextStorage<'c, C>
	where
		C: YitContext,
	{
		type Encoded = PlainTextEncoded;

		fn state(&self) -> &'c C {
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

pub mod toml {
	use std::str::FromStr as _;

	use crate::{
		YitContext,
		hash::ForwardsCompatHash,
		prelude::*,
		storage::{EncodedStorage, Stateful, Storage},
	};

	#[derive(Debug)]
	pub struct TomlStorage<'c, C> {
		state: &'c C,
		// strict_parse_utf8: bool,
	}

	#[derive(Debug)]
	pub struct TomlEncoded(pub toml_edit::DocumentMut);

	impl ForwardsCompatHash for TomlEncoded {
		fn prefix(&self) -> &'static [u8] {
			b"https://docs.rs/yit/latest/yit/storage/toml"
		}
		fn hash<H: crate::hash::MinimalHasher + ?Sized>(&self, hasher: &mut H) {
			hasher.write(self.prefix());
			// could use different method of hashing involving lots of forwards compatability
			hasher.write(&self.0.clone().to_string().into_bytes());
		}
	}

	impl EncodedStorage for TomlEncoded {}

	impl<'c, C> Storage<'c, C> for TomlStorage<'c, C>
	where
		C: YitContext,
	{
		type Encoded = TomlEncoded;

		fn state(&self) -> &'c C {
			self.state
		}

		async fn decode(&self, data: Vec<u8>) -> color_eyre::Result<Self::Encoded> {
			let str = String::from_utf8(Vec::from(data))
				.wrap_err("Couldn't parse data strictly as UTF8")?;
			toml_edit::DocumentMut::from_str(&str)
				.wrap_err("Couldn't parse data as TOML")
				.map(TomlEncoded)
		}
		async fn encode(&self, encoded: Self::Encoded) -> Vec<u8> {
			encoded.0.to_string().into_bytes()
		}
	}
}
