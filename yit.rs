use clap::Parser as _;
use yit::prelude::*;
use ystd::{env, prelude::*};

#[derive(clap::Parser, Debug)]
pub struct Cli {
	#[clap(subcommand)]
	pub cmd: Cmd,
}

#[derive(clap::Subcommand, Debug)]
pub enum Cmd {
	Hash { path: Utf8PathBuf },
}

fn main() -> color_eyre::Result<()> {
	tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.thread_stack_size(32 * 1024 * 1024) // 32 MiB
		.build()
		.unwrap()
		.block_on(async { _main().await })?;

	Ok(())
}

// #[tokio::main]
async fn _main() -> color_eyre::Result<()> {
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
