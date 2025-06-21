use yit::prelude::*;
use ystd::prelude::*;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	yit::app_tracing::install_tracing("info,yit=trace").await?;
	trace!("Started yit tracing");

	Ok(())
}
