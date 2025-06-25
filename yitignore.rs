use yit::YitContext;
use ystd::prelude::*;

pub async fn yitignore(root: &YitContext, path: impl AsRef<Utf8Path>) -> color_eyre::Result<bool> {
	let path = root.resolve_local_path(path).await?;
	let excluded_paths = [".yit", "target"].into_iter().map(|tl| root.dir().join(tl));
	for excluded_path in excluded_paths {
		if path.starts_with(&excluded_path) {
			return Ok(true);
		} else {
			warn!(?path, ?excluded_path);
		}
	}

	Ok(false)
}
