use clap::Parser;
use xtask::cli;
use xtask::prelude::*;
use xtask::ServerSession;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let args = xtask::cli::Args::parse();

    match args.cmd {
        cli::Commands::Server(server_command) => {
            let server = ServerSession::connect_to_server().await?;
            match server_command {
                cli::ServerCommands::Check => match server.is_surreal_running().await? {
                    true => info!("Database is running"),
                    false => info!("Database is not running"),
                },
                cli::ServerCommands::Start => {}
            }
        }
    }

    Ok(())
}
