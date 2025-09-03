use tracing::*;

fn main() {
	yeditor::init_debug_tools("yeditor=debug").unwrap();
	debug!("Logging started");

	info!("Hello, world!");
}
