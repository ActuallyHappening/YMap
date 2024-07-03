/// Includes an asset from the root of the project
#[macro_export]
macro_rules! include_asset {
	($name:literal) => {
		include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $name))
	};
}
