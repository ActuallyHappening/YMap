use yit::YitRoot;
use ystd::prelude::*;

pub async fn yitignore(root: YitRoot, path: impl AsRef<Utf8Path>) -> color_eyre::Result<bool> {
	let path = root.resolve_local_path(path).await?;
	let excluded_dirs = [".yit", "target"];

	todo!()
}
