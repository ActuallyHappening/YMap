use std::process::Command;

use clap::Parser;
use tracing::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Args {
    Run,
		List,
}

fn main() {
    tracing_subscriber::fmt::init();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let parent_dir = std::path::Path::new(&manifest_dir).parent().unwrap();
		// set cwd to parent_dir
		std::env::set_current_dir(parent_dir).unwrap();

    let args = Args::parse();
    match args {
        Args::Run => {
            info!("Running ...");
            // execute nu run.nu
            let process = Command::new("nu")
                .arg("run.nu")
                .spawn()
                .expect("failed to execute process");
            process.wait_with_output().unwrap();
        }
				Args::List => {
					info!("Listing devices ...");
					let process = Command::new("xcrun").args(["xctrace", "list", "devices"]).spawn().expect("failed to execute process");
					process.wait_with_output().unwrap();
				}
    }
}
