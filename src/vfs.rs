use std::{borrow::Cow, collections::HashMap};

use ystd::path::FileTypeExhaustive;

use crate::{
	YitContext,
	prelude::*,
	storage::{self, File, Storage},
};

pub(crate) type Key = Cow<'static, str>;

pub struct Vfs<S, C>
where
	S: Storage<C>,
	C: YitContext,
{
	pub files: Vec<storage::File<S, C>>,
	pub folders: HashMap<Key, Vfs<S, C>>,
}

impl<S, C> core::fmt::Debug for Vfs<S, C>
where
	S: Storage<C>,
	C: YitContext,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Vfs")
			.field("files", &self.files)
			.field("folders", &self.folders)
			.finish()
	}
}

impl<S, C> Vfs<S, C>
where
	S: Storage<C>,
	C: YitContext,
{
	pub async fn snapshot_dir(
		// root: &impl YitContext,
		storage: &S,
		dir: impl AsRef<Utf8Path>,
	) -> color_eyre::Result<Vfs<S, C>> {
		let state = storage.state();
		let dir = state.resolve_local_path(dir).await?;
		dir.assert_dir().await?;

		let mut files = Vec::new();
		let mut folders = HashMap::new();

		let top_level_files = dir.read_dir_utf8().await?;
		for entry in top_level_files {
			let entry = entry?;
			if storage.state().is_ignored(entry.path()).await? {
				continue;
			}
			match entry.path().file_type_exhaustive().await? {
				FileTypeExhaustive::File => {
					// single file
					let file = Box::pin(File::snapshot(storage, entry.path())).await?;
					files.push(file);
				}
				FileTypeExhaustive::Dir => {
					// recursive
					let vfs = Box::pin(Vfs::snapshot_dir(storage, entry.path())).await?;
					folders.insert(Cow::Owned(entry.file_name().to_owned()), vfs);
				}
			}
		}

		Ok(Vfs { files, folders })
	}
}
