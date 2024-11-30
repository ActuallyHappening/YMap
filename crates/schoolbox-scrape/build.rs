use std::{fs::File, path::PathBuf};

fn main() {
    let path: PathBuf = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap()
        .parse::<PathBuf>()
        .unwrap()
        .join("src")
        .join("cookie.txt");
    if !path.exists() {
        File::create_new(path).unwrap();
    }
}
