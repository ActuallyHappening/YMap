use clap::Parser as _;
use yit::{DefaultYitContext, YitContext, prelude::*};
use ystd::{env, prelude::*};

mod yitignore;

#[derive(clap::Parser, Debug)]
pub struct Cli {
	#[clap(subcommand)]
	pub cmd: Cmd,
}

#[derive(clap::Subcommand, Debug)]
pub enum Cmd {
	State,
	#[clap(subcommand)]
	Plumbing(Plumbing),
}

#[derive(clap::Subcommand, Debug)]
pub enum Plumbing {
	Hash { path: Utf8PathBuf },
	IsIgnored { path: Utf8PathBuf },
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	yit::app_tracing::install_tracing("info,yit=trace").await?;
	trace!("Started yit tracing");

	let root = DefaultYitContext::new(env::current_dir().await?).await?;
	let cli = Cli::parse();

	match cli.cmd {
		Cmd::State => {
			todo!()
		}
		Cmd::Plumbing(cmd) => match cmd {
			Plumbing::Hash { path } => {
				let hash = yit::hash::debug_hash_from_path(&path).await?;
				info!(?hash);
			}
			Plumbing::IsIgnored { path } => {
				let path = root.resolve_local_path(path).await?;
				let ignored_attr = yitignore::yitignore(&root, &path).await?;
				info!(?ignored_attr, ?path);
			}
		},
	}

	Ok(())
}
