use serde::{Deserialize, de::DeserializeOwned};

use crate::{
	prelude::*,
	vfs::{self, Vfs},
};

pub struct YitContext {
	/// Canonicalized
	dir: Utf8PathBuf,
	type_registry: bevy_reflect::TypeRegistry,
}

impl YitContext {
	pub async fn new(dir: impl AsRef<Utf8Path>) -> Result<Self> {
		let dir = dir.as_ref().to_path_buf();
		let dir = dir.canonicalize().await?;
		dir.assert_dir().await?;
		Ok(Self {
			dir,
			type_registry: bevy_reflect::TypeRegistry::default(),
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

	pub async fn resolve_attr<A>(&self, path: impl AsRef<Utf8Path>) -> color_eyre::Result<A> {
		let path = self.resolve_local_path(path).await?;
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
