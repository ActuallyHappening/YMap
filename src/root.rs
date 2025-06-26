use std::{any::Any, sync::Arc};

use serde::{Deserialize, de::DeserializeOwned};
use tokio::task::JoinHandle;

use crate::{
	prelude::*,
	vfs::{self, Vfs},
};

pub trait YitContext: Sized {
	fn dir(&self) -> &Utf8Path;
	fn registry(&self) -> &bevy_reflect::TypeRegistry;

	/// Makes sure the path provided is within this Yit root directory
	async fn resolve_local_path(
		&self,
		path: impl AsRef<Utf8Path>,
	) -> color_eyre::Result<Utf8PathBuf> {
		let path = path.as_ref().canonicalize().await?;
		for ancestor in path.ancestors() {
			if *ancestor == *self.dir() {
				return Ok(path);
			}
		}
		bail!(
			"Path {} isn't within the yit project root of {}",
			path,
			self.dir()
		);
	}

	async fn is_ignored(&self, path: impl AsRef<Utf8Path>) -> color_eyre::Result<bool>;

	async fn snapshot(&self) -> color_eyre::Result<vfs::Vfs> {
		vfs::Vfs::snapshot_dir(&self, &self.dir()).await
	}
}

impl<T> YitContext for &T
where
	T: YitContext,
{
	fn dir(&self) -> &Utf8Path {
		(*self).dir()
	}

	fn registry(&self) -> &bevy_reflect::TypeRegistry {
		(*self).registry()
	}

	async fn is_ignored(&self, path: impl AsRef<Utf8Path>) -> color_eyre::Result<bool> {
		(*self).is_ignored(path).await
	}
}

pub struct DefaultYitContext<Ignored> {
	/// Canonicalized
	dir: Utf8PathBuf,
	type_registry: bevy_reflect::TypeRegistry,
	ignored: Ignored,
}

pub trait YitIgnore {
	async fn ignored(&self, state: &impl YitContext, path: &Utf8Path) -> color_eyre::Result<bool>;
}

pub struct IgnoreNothing;

impl YitIgnore for IgnoreNothing {
	async fn ignored(
		&self,
		_state: &impl YitContext,
		_path: &Utf8Path,
	) -> color_eyre::Result<bool> {
		Ok(false)
	}
}

impl DefaultYitContext<IgnoreNothing> {
	pub async fn new(dir: impl AsRef<Utf8Path>) -> Result<Self> {
		let dir = dir.as_ref().to_path_buf();
		let dir = dir.canonicalize().await?;
		dir.assert_dir().await?;
		Ok(Self {
			dir,
			type_registry: bevy_reflect::TypeRegistry::default(),
			ignored: IgnoreNothing,
		})
	}
}

impl<Ignored> DefaultYitContext<Ignored> {
	pub fn with_ignored<I>(self, ignore: I) -> DefaultYitContext<I>
	where
		I: YitIgnore + 'static,
	{
		DefaultYitContext {
			dir: self.dir,
			type_registry: self.type_registry,
			ignored: ignore,
		}
	}
}

impl<Ignored> YitContext for DefaultYitContext<Ignored>
where
	Ignored: YitIgnore,
{
	fn dir(&self) -> &Utf8Path {
		&self.dir
	}

	fn registry(&self) -> &bevy_reflect::TypeRegistry {
		&self.type_registry
	}

	/// Makes sure the path provided is within this Yit root directory
	async fn resolve_local_path(
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

	async fn is_ignored(&self, path: impl AsRef<Utf8Path>) -> color_eyre::Result<bool> {
		self.ignored.ignored(&self, path.as_ref()).await
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn resolve_local_path() -> color_eyre::Result<()> {
		let root = DefaultYitContext::new(env!("CARGO_MANIFEST_DIR")).await?;
		let path = root.dir().join("src").join("lib.rs");
		eyre_assert_eq!(root.resolve_local_path(&path).await?, path);
		Ok(())
	}
}
