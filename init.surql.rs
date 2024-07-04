#[allow(dead_code)]
pub const INIT_SURQL: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/init.surql"));

#[test]
fn db_init_is_valid() {
	let file_path = camino::Utf8PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/init.surql"));
	bossy::Command::impure("surreal")
		.with_args(["validate", file_path.as_str()])
		.run_and_wait()
		.unwrap();
}
