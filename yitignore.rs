use yit::{YitContext, YitIgnore};
use ystd::prelude::*;

pub struct MyIgnore;

impl YitIgnore for MyIgnore {
	async fn ignored(&self, state: &impl YitContext, path: &Utf8Path) -> color_eyre::Result<bool> {
		let path = state.resolve_local_path(path).await?;
		let excluded_paths = [".yit", "target", ".git"]
			.into_iter()
			.map(|tl| state.dir().join(tl));
		for excluded_path in excluded_paths {
			if path.starts_with(&excluded_path) {
				return Ok(true);
			} else {
				warn!(?path, ?excluded_path);
			}
		}

		Ok(false)
	}
}
