use tracing::*;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	yeditor::app_tracing::init_debug_tools("yeditor=debug").unwrap();
	debug!("Logging started");

	yeditor::main().await?;

	debug!("Cleanly exitted");
	Ok(())
}
