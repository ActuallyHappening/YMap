use std::{borrow::Cow, collections::HashMap};

use ystd::path::FileTypeExhaustive;

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

		let mut files = Vec::new();
		let mut folders = HashMap::new();

		let top_level_files = dir.read_dir_utf8().await?;
		for entry in top_level_files {
			let entry = entry?;
			match entry.path().file_type_exhaustive().await? {
				FileTypeExhaustive::File => {
					todo!()
				}
				FileTypeExhaustive::Dir => {
					// recursive
					let vfs = Box::pin(Vfs::snapshot_dir(&root, entry.path())).await?;
					folders.insert(Cow::Owned(entry.file_name().to_owned()), vfs);
				}
			}
		}

		Ok(Vfs { files, folders })
	}
}
