pub use errors::{Error, Result};
use serde::{Deserialize, Serialize};
use tracing::*;
mod errors {
    pub type Result<T> = color_eyre::eyre::Result<T>;
    pub type Error = color_eyre::eyre::Report;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub email: String,
}

impl Person {
    pub async fn save_to_db<C: surrealdb::Connection>(
        self,
        db: &surrealdb::Surreal<C>,
    ) -> Result<Person> {
        debug!(message = "Saving person to db", ?self);
        Ok(db.create("people").content(self).await?.unwrap())
    }

    pub fn find_in_document(document: &scraper::Html) -> Result<Person> {
        let name = find_name(document)?;
        let email = find_email(document)?;
        Ok(Person { name, email })
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
