use std::collections::HashMap;

use color_eyre::eyre::ContextCompat;
pub use errors::{Error, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tracing::*;
mod errors {
    pub type Result<T> = color_eyre::eyre::Result<T>;
    pub type Error = color_eyre::eyre::Report;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub email: Option<String>,
    pub schoolbox_id: u32,
    /// If present, we know for sure
    pub is_student: Option<bool>,
    /// Should be present on all teachers,
    /// but the admin is an exception
    pub teacher_department: Option<String>,
}

impl Person {
    pub async fn save_to_db<C: surrealdb::Connection>(
        self,
        db: &surrealdb::Surreal<C>,
    ) -> Result<Person> {
        debug!(message = "Saving person to db", ?self);
        Ok(db.create("people").content(self).await?.unwrap())
    }

    pub fn find_in_document(document: &scraper::Html, num: u32) -> Result<Person> {
        let name = find_name(&document)?;
        let profile = find_profile_values(&document)?;

        let email = match (profile.get("Email"), profile.get("Email:")) {
            (Some(_), Some(_)) => {
                return Err(Error::msg("Both Email and Email: are present?"));
            }
            (Some(email), None) | (None, Some(email)) => Some(
                email
                    .child_elements()
                    .next()
                    .wrap_err("Child Email has no inner link")?
                    .text()
                    .collect::<String>()
                    .trim()
                    .to_string(),
            ),
            (None, None) => None,
        };

        let mut person = Person {
            name,
            email,
            schoolbox_id: num,
            is_student: None,
            teacher_department: None,
        };

        // applies email heuristic to determine if student or teacher
        if person.is_email_student() && !person.is_email_teacher() {
            person.is_student = Some(true);
        }
        if person.is_email_teacher() && !person.is_email_student() {
            person.is_student = Some(false);
        }

        if person.is_student == Some(false) {
            person.teacher_department = profile
                .get("Department:")
                .map(|el| el.text().collect::<String>().trim().to_string());
        }

        Ok(person)
    }

    /// Conservative
    fn is_email_student(&self) -> bool {
        self.email
            .as_ref()
            .is_some_and(|email| email.ends_with("@students.emmanuel.qld.edu.au"))
    }

    /// Conservative
    fn is_email_teacher(&self) -> bool {
        self.email
            .as_ref()
            .is_some_and(|email| email.ends_with("@emmanuel.qld.edu.au"))
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum ScrapeChunk {
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

fn cookie() -> String {
    const COOKIE: &str = include_str!("cookie.txt");
    COOKIE.trim().into()
}

pub async fn get_website(client: &reqwest::Client, num: u32) -> Result<String> {
    debug!(message = "Getting student", %num);
    let url = format!(
        "https://schoolbox.emmanuel.qld.edu.au/search/user/{num}",
        num = num
    );

    client
        .get(&url)
        .header(reqwest::header::COOKIE, cookie())
        .send()
        .await?
        .text()
        .await
        .map_err(Error::from)
}

fn find_profile_values(
    document: &scraper::Html,
) -> Result<HashMap<String, scraper::ElementRef<'_>>> {
    let selector =
        scraper::Selector::parse(r##"div.profile.content>div.row>div.columns>dl"##).unwrap();
    let profile = document
        .select(&selector)
        .next()
        .wrap_err("No profile found")?;
    trace!(?profile, "Raw profile");
    let mut profile = profile.child_elements().tuples();
    let mut values = HashMap::new();
    while let Some((key, value)) = profile.next() {
        values.insert(key.text().collect::<String>(), value);
    }
    Ok(values)
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
