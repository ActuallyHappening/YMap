use clap::Parser;
use xtask::*;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let args = xtask::cli::Args::parse();

    match args.cmd {
        cli::Commands::Server(server_command) => match server_command {
            cli::ServerCommands::Start => {
                let server = connect_to_server().await?;
                
            }
        },
    }

    Ok(())
}
