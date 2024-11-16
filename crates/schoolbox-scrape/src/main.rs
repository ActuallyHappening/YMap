use clap::{Parser, Subcommand};
use itertools::Itertools;
use schoolbox_scrape::{get_website, Person, Result};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tokio::task::JoinSet;
use tracing::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, default_value_t = 20)]
    chunk_size: usize,

    #[command(subcommand)]
    command: ScrapeChunk,
}

#[derive(Debug, Subcommand)]
enum ScrapeChunk {
    Single {
        num: u32,
    },
    Range {
        #[arg(long)]
        start: u32,
        #[arg(long)]
        end: u32,
    },
}

impl ScrapeChunk {
    pub fn split_into_chunks(self, chunk_size: usize) -> Vec<ScrapeChunk> {
        match self {
            ScrapeChunk::Single { num } => vec![ScrapeChunk::Single { num }],
            ScrapeChunk::Range { start, end } => (start..=end)
                .chunks(chunk_size)
                .into_iter()
                .map(|mut chunk| {
                    let first = chunk.next().unwrap();
                    match chunk.last() {
                        None => ScrapeChunk::Single { num: first },
                        Some(last) => ScrapeChunk::Range {
                            start: first,
                            end: last,
                        },
                    }
                })
                .collect(),
        }
    }
}

impl IntoIterator for ScrapeChunk {
    type Item = u32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ScrapeChunk::Single { num } => vec![num].into_iter(),
            ScrapeChunk::Range { start, end } => (start..=end).collect::<Vec<_>>().into_iter(),
        }
    }
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

    info!("Starting scraping ...");

    let mut join_set = JoinSet::new();
    for chunk in cli
        .command
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
