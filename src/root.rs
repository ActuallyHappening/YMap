use std::sync::Arc;

use serde::{Deserialize, de::DeserializeOwned};

use crate::{
	prelude::*,
	vfs::{self, Vfs},
};

pub struct YitContext {
	/// Canonicalized
	dir: Utf8PathBuf,
	type_registry: bevy_reflect::TypeRegistry,
	pub ignored: Box<dyn YitIgnore>,
}

pub trait YitIgnore {
	async fn ignored(&self, state: &YitContext);
}

pub struct BoxFut<O: 'static>(pub Box<dyn Future<Output = O>>);

pub struct YitIgnore<Fut> {
	inner: Box<dyn Fn(Arc<YitContext>, Utf8PathBuf) -> Fut>,
}

impl YitIgnore {
	pub fn ignore_nothing() -> YitIgnore {
		fn nothing_ignored(
			root: Arc<YitContext>,
			path: Utf8PathBuf,
		) -> Box<dyn Future<Output = color_eyre::Result<bool>>> {
			Box::new(async { Ok(false) })
		}
		Self::from_cb(nothing_ignored)
	}

	pub fn from_cb<Fut>(cb: impl Fn(Arc<YitContext>, Utf8PathBuf) -> Fut + 'static) -> YitIgnore
	where
		Fut: Future<Output = color_eyre::Result<bool>> + 'static,
	{
		let cb = move |root, path| {
			let fut: Fut = cb(root, path);
			return BoxFut(Box::new(fut));
		};
		YitIgnore::from_inner(cb)
	}

	pub fn from_inner(
		cb: impl Fn(Arc<YitContext>, Utf8PathBuf) -> BoxFut<color_eyre::Result<bool>> + 'static,
	) -> YitIgnore {
		Self {
			inner: Box::new(cb)
				as Box<dyn Fn(Arc<YitContext>, Utf8PathBuf) -> BoxFut<color_eyre::Result<bool>>>,
		}
	}

	pub async fn is_ignored(
		&self,
		root: Arc<YitContext>,
		path: impl AsRef<Utf8Path>,
	) -> color_eyre::Result<bool> {
		let path = path.as_ref().to_owned();
		let fut = Box::into_pin((self.inner)(root, path));
		fut.await
	}
}

impl YitContext {
	pub async fn new(dir: impl AsRef<Utf8Path>) -> Result<Self> {
		let dir = dir.as_ref().to_path_buf();
		let dir = dir.canonicalize().await?;
		dir.assert_dir().await?;
		Ok(Self {
			dir,
			type_registry: bevy_reflect::TypeRegistry::default(),
			ignored: YitIgnore::ignore_nothing(),
		})
	}

	pub fn dir(&self) -> &Utf8Path {
		&self.dir
	}

	pub fn registry(&self) -> &bevy_reflect::TypeRegistry {
		&self.type_registry
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

	pub fn set_ignored(&mut self, ignore: YitIgnore) -> &mut Self {
		self.ignored = ignore;
		self
	}

	pub fn with_ignored(mut self, ignore: YitIgnore) -> Self {
		self.ignored = ignore;
		self
	}

	pub async fn is_ignored(&self, path: impl AsRef<Utf8Path>) -> color_eyre::Result<bool> {
		todo!()
	}

	pub async fn snapshot(&self) -> color_eyre::Result<vfs::Vfs> {
		vfs::Vfs::snapshot_dir(&self, &self.dir).await
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn resolve_local_path() -> color_eyre::Result<()> {
		let root = YitContext::new(env!("CARGO_MANIFEST_DIR")).await?;
		let path = root.dir().join("src").join("lib.rs");
		eyre_assert_eq!(root.resolve_local_path(&path).await?, path);
		Ok(())
	}
}
