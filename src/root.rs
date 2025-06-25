
use crate::prelude::*;

	pub struct YitRoot {
		/// Canonicalized
		dir: Utf8PathBuf,
	}

	impl YitRoot {
		pub async fn new(dir: impl AsRef<Utf8Path>) -> Result<Self> {
			let dir = dir.as_ref().to_path_buf();
			let dir = dir.canonicalize().await?;
			dir.assert_dir().await?;
			Ok(Self { dir })
		}

		pub fn dir(&self) -> &Utf8Path {
			&self.dir
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
		
		pub async fn snapshot(&self) -> 
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[tokio::test]
		async fn resolve_local_path() -> color_eyre::Result<()> {
			let root = YitRoot::new(env!("CARGO_MANIFEST_DIR")).await?;
			let path = root.dir().join("src").join("lib.rs");
			eyre_assert_eq!(root.resolve_local_path(&path).await?, path);
			Ok(())
		}
	}
