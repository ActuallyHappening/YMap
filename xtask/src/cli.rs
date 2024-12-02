use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about)]
pub struct Cli {
	#[command(subcommand)]
	pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
	#[command(subcommand)]
	Server(ServerCommands),
}

#[derive(Subcommand, Debug)]
pub enum ServerCommands {
	Scan,
	Start {
		#[command(flatten)]
		args: CommandSpawnOptions,
	},
	Kill,
	Clean,
}

/// Options to control processes being spawned that don't terminate immediately
#[derive(Args, Debug, Clone)]
pub struct CommandSpawnOptions {
	/// If present, will run the start command in the foreground.
	/// Useful for debugging errors, but upon exiting this process the
	/// server process will also exit.
	///
	/// Defaults to running in the background.
	#[arg(long, default_value_t = false)]
	pub foreground: bool,
}

impl CommandSpawnOptions {
	pub fn in_background(&self) -> bool {
		!self.foreground
	}
}
