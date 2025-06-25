use std::{borrow::Cow, collections::HashMap};

use crate::{YitRoot, prelude::*, storage};

pub(crate) type Key = Cow<'static, str>;

pub struct Vfs {
	pub files: Vec<storage::File>,
	pub folders: HashMap<Key, Vfs>,
}

impl Vfs {
	pub async fn snapshot_dir(
		root: &YitRoot,
		dir: impl AsRef<Utf8Path>,
	) -> color_eyre::Result<Vfs> {
		let dir = root.resolve_local_path(dir).await?;
		dir.assert_dir().await?;

		todo!()
	}
}
