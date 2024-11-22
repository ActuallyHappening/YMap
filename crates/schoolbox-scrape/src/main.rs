use clap::{Args, Parser, Subcommand};
use schoolbox_scrape::{get_website, Person, Result, ScrapeChunk};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::{Connection, Surreal};
use tokio::task::JoinSet;
use tracing::*;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    Scrape(#[command(subcommand)] ScrapeCommand),
    ExportDB {
        #[arg(long, default_value_t = camino::Utf8PathBuf::from("./people.csv"))]
        output_path: camino::Utf8PathBuf,
    },
}

#[derive(Debug, Args)]
struct ScrapeCommand {
    #[arg(long, default_value_t = 20)]
    chunk_size: usize,

    #[command(subcommand)]
    chunk: ScrapeChunk,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    let cli = Cli::parse();

    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    db.use_ns("primary").use_db("primary").await?;

    match cli.command {
        SubCommand::Scrape(scrap_command) => scrape_users(scrap_command, &db).await?,
        SubCommand::ExportDB { output_path } => export_db(&db, &output_path).await?,
    }

    Ok(())
}

async fn scrape_users<C: Connection>(cli: ScrapeCommand, db: &Surreal<C>) -> Result<()> {
    info!("Starting scraping ...");

    let mut join_set = JoinSet::new();
    for chunk in cli
        .chunk
        .split_into_chunks(cli.chunk_size)
        .into_iter()
        .inspect(|chunk| info!(?chunk, "Scraping chunk"))
    {
        let f = async move {
            let client = reqwest::Client::new();

            let nums = chunk.into_iter();
            let mut results = Vec::with_capacity(nums.len());
            for num in nums {
                let result = scrape_user(&client, num).await;
                match &result {
                    Ok(person) => {
                        info!(?person, "Successfully scraped person {num}");
                    }
                    Err(e) => {
                        warn!("Failed to scrape person {num}: {}", e);
                    }
                };
                results.push(result);
            }

            results
        };
        join_set.spawn(f);
    }

    let people_to_save = join_set
        .join_all()
        .await
        .into_iter()
        .flatten()
        .filter_map(|r| r.ok());
    for person in people_to_save {
        person.save_to_db(&db).await?;
    }

    info!("Finished scraping");

    Ok(())
}

async fn scrape_user(client: &reqwest::Client, num: u32) -> Result<Person> {
    let website = get_website(&client, num).await?;
    let document = scraper::Html::parse_document(&website);

    let person = Person::find_in_document(&document, num)?;
    drop(document);
    // person.save_to_db(&db).await
    Ok(person)
}

async fn export_db<C: Connection>(db: &Surreal<C>, output_path: &camino::Utf8Path) -> Result<()> {
    let people: Vec<Person> = db.select("people").await?;
    let mut writer = csv::Writer::from_path(output_path)?;
    for person in people {
        writer.serialize(person)?;
    }
    writer.flush()?;
    Ok(())
}
