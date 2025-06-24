use crate::prelude::*;

pub struct FileMatching {}

impl FileMatching {
	pub async fn new(yit_root: impl AsRef<Utf8Path>) -> Result<FileMatching> {
		let yit_root = yit_root.as_ref();
		let file = yit_root.join("matching.rs");

		if !file.is_file() {
			return Err(eyre!("File path {file} isn't a file"));
		}

		todo!()
	}
}
