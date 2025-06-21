use clap::Parser as _;
use yit::prelude::*;
use ystd::prelude::*;

#[derive(clap::Parser, Debug)]
pub struct Cli {
	#[clap(subcommand)]
	pub cmd: Cmd,
}

#[derive(clap::Subcommand, Debug)]
pub enum Cmd {
	Hash { path: Utf8PathBuf },
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	yit::app_tracing::install_tracing("info,yit=trace").await?;
	trace!("Started yit tracing");

	let cli = Cli::parse();

	match cli.cmd {
		Cmd::Hash { path } => {
			let hash = yit::hash::hash(&path).await?;
			info!(?hash);
		}
	}

	Ok(())
}
