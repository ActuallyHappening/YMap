use clap::Parser;
use xtask::ServerSession;
use xtask::cli;
use xtask::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let args = xtask::cli::Cli::parse();

    match args.cmd {
        cli::Commands::Server(server_command) => {
            let server = ServerSession::connect_to_server().await?;
            match server_command {
                cli::ServerCommands::Scan => match server.scan().await? {
                    true => info!("Database is running"),
                    false => info!("Database is not running"),
                },
                cli::ServerCommands::Start { args } => {
                    server.start(args).await?;
                    if server.scan().await? == false {
                        warn!("Started surreal, but surreal is not running?");
                    } else {
                        info!("Started surreal successfully");
                    }
                }
                cli::ServerCommands::Kill => {
                    server.kill().await?;
                    if server.scan().await? == true {
                        warn!("Killed surreal, but surreal is still running?");
                    } else {
                        info!("Killed surreal successfully");
                    }
                }
                cli::ServerCommands::Clean => {
                    server.clean().await?;
                    info!("Cleaned surreal successfully");
                }
            }
        }
    }

    Ok(())
}
