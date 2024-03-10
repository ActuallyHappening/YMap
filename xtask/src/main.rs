use anyhow::Context;
use clap::Parser;
use std::process::Command;
use tracing::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Args {
    Run,
    List,
    Connect,
}

fn main() -> anyhow::Result<()> {
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
                .context("failed to execute process")?;
            process.wait_with_output().context("nu failed")?;
        }
        Args::List => {
            info!("Listing devices ...");
            let process = Command::new("xcrun")
                .args(["xctrace", "list", "devices"])
                .spawn()
                .context("failed to execute process")?;
            process.wait_with_output().context("xcrun failed")?;
        }
        Args::Connect => {
            info!("Connecting to local device ...");
            Command::new("cargo")
                .args(["bundle", "--target", "aarch64-apple-ios"])
                .spawn()
                .context("failed to execute process")?
                .wait_with_output()
                .context("cargo failed")?;
            Command::new("xcrun")
                .args([
                    "simctl",
                    "install",
                    "AH iPad Pro (17.3.1)",
                    &format!(
                        "target/aarch64-apple-ios-sim/debug/bundle/ios/{APP_NAME}.app",
                        APP_NAME = "infi-map"
                    ),
                ])
                .spawn()
                .context("failed to execute process")?
                .wait_with_output()
                .context("failed")?;
        }
    }

    Ok(())
}
