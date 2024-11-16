use schoolbox_scrape::{get_website, Result};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let client = reqwest::Client::builder().cookie_store(true).build()?;

    let num = 1133;
    let website = get_website(&client, num).await?;
    info!(%website);

    Ok(())
}
