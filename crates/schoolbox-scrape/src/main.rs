use schoolbox_scrape::{find_email, find_name, get_website, Person, Result};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tracing::info;

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

    let num = 1133;
    let website = get_website(&client, num).await?;
    let document = scraper::Html::parse_document(&website);

    let name = find_name(&document)?;
    let email = find_email(&document)?;
    let person = Person { name, email };
    person.save_to_db(&db).await?;

    Ok(())
}
