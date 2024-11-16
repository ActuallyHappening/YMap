use schoolbox_scrape::{get_website, Person, Result};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tracing::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    db.use_ns("primary").use_db("primary").await?;

    info!("Starting scraping ...");

    for num in 0..=2000 {
        let result = scrape_user(&client, &db, num).await;
        match result {
            Ok(person) => info!(?person, "Successfully scraped person {num}",),
            Err(e) => warn!("Failed to scrape person {num}: {}", e),
        }
    }

    info!("Finished scraping");
    Ok(())
}

async fn scrape_user<C: surrealdb::Connection>(
    client: &reqwest::Client,
    db: &surrealdb::Surreal<C>,
    num: u32,
) -> Result<Person> {
    let website = get_website(&client, num).await?;
    let document = scraper::Html::parse_document(&website);

    let person = Person::find_in_document(&document, num)?;
    person.save_to_db(&db).await
}
