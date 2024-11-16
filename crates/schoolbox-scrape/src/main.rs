use schoolbox_scrape::{get_website, Result};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let client = reqwest::Client::builder().cookie_store(true).build()?;

    let num = 1133;
    let website = get_website(&client, num).await?;
    let document = scraper::Html::parse_document(&website);

    let name = find_name(&document)?;
    let email = find_email(&document)?;

    info!(%name, %email);

    Ok(())
}

fn find_name(document: &scraper::Html) -> Result<String> {
    let selector = scraper::Selector::parse(r##"h1[data-test="user-profile-name"]"##).unwrap();
    let mut names = document.select(&selector);
    match names.next() {
        None => Err(color_eyre::eyre::eyre!("No name found")),
        Some(name) => match names.next() {
            Some(another_name) => Err(color_eyre::eyre::eyre!(
                "Multiple names found: {:?} and {:?}",
                name.value(),
                another_name.value()
            )),
            None => Ok(name.text().collect::<String>()),
        },
    }
}

fn find_email(document: &scraper::Html) -> Result<String> {
    let selector =
        scraper::Selector::parse(r##"div.profile.content>div.row>div.columns>dl>dd>a[href]"##)
            .unwrap();
    let mut email = document.select(&selector);
    match email.next() {
        None => Err(color_eyre::eyre::eyre!("No email found")),
        // if signed in, other similar links appear
        Some(email) => Ok(email.text().collect::<String>()),
    }
}
