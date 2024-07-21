use clap::Parser;
use docker_api::Docker;
use std::path::PathBuf;

#[cfg(unix)]
pub fn new_docker() -> Result<Docker, docker_api::Error> {
	Ok(Docker::unix("/var/run/docker.sock"))
}

#[cfg(not(unix))]
pub fn new_docker() -> Result<Docker> {
	Docker::new("tcp://127.0.0.1:8080")
}

#[derive(Parser)]
pub struct Opts {
	#[command(subcommand)]
	subcommand: Cmd,
}

impl Opts {
	pub fn subcommand(&self) -> &Cmd {
		&self.subcommand
	}
}

#[derive(Parser)]
pub enum Cmd {
	/// Attach to a running containers TTY.
	Attach { id: String },
	/// Copy files from a container.
	CopyFrom {
		id: String,
		remote_path: PathBuf,
		local_path: PathBuf,
	},
	/// Copy files into a container.
	CopyInto {
		local_path: PathBuf,
		id: String,
		remote_path: PathBuf,
	},
	/// Create a new image from a container
	Commit {
		/// Container ID
		id: String,
		#[arg(short, long)]
		/// Repository name for the created image
		repo: Option<String>,
		#[arg(short, long)]
		/// Tag name for the create image
		tag: Option<String>,
		#[arg(short, long)]
		/// Commit message
		comment: Option<String>,
		#[arg(short, long)]
		/// Author of the image (e.g., John Hannibal Smith <hannibal@a-team.com>)
		author: Option<String>,
		#[arg(short, long)]
		///  Whether to pause the container before committing
		pause: Option<bool>,
		#[arg(long)]
		/// Dockerfile instructions to apply while committing
		changes: Option<String>,
	},
	/// Create a new container.
	Create {
		image: String,
		#[arg(short, long = "name")] // for some reason naming field `name` makes clap error. Possibly a bug?
		/// The name of the container to create.
		nam: Option<String>,
	},
	/// Delete an existing container.
	Delete {
		id: String,
		#[arg(short, long)]
		force: bool,
	},
	/// Execute a command in a running container.
	Exec { id: String, cmd: Vec<String> },
	/// Inspect a container.
	Inspect { id: String },
	/// List active containers.
	List {
		#[arg(long, short)]
		/// List stopped and running containers.
		all: bool,
	},
	/// Print logs of a container.
	Logs {
		id: String,
		#[arg(long)]
		stdout: bool,
		#[arg(long)]
		stderr: bool,
	},
	/// Delete stopped containers.
	Prune {
		#[arg(long)]
		/// Prune containers before this timestamp. Can be a unix timestamp or duration
		/// string like `1h30m`
		until: Option<String>,
	},
	/// Get information about a file in container.
	StatFile { id: String, path: PathBuf },
	/// Stops the container
	Stop {
		id: String,
		#[arg(long)]
		/// Time in seconds to wait before stopping the container
		wait: Option<usize>,
		#[arg(long)]
		/// Example `SIGINT`
		signal: Option<String>,
	},
	/// Restarts the container
	Restart {
		id: String,
		#[arg(long)]
		/// Time in seconds to wait before restarting the container
		wait: Option<usize>,
		#[arg(long)]
		/// Example `SIGINT`
		signal: Option<String>,
	},
	/// Returns usage statistics of the container.
	Stats { id: String },
	/// Returns information about running processes in the container.
	Top {
		id: String,
		/// Arguments passed to `ps` in the container.
		psargs: Option<String>,
	},
}
