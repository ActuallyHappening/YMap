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
    pub schoolbox_id: u32,
    /// If present, we know for sure
    pub is_student: Option<bool>,
    /// Should be present on all teachers
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

    /// Should be followed by [Person::set_schoolbox_id]
    pub fn find_in_document(document: &scraper::Html, num: u32) -> Result<Person> {
        let name = find_name(document)?;
        let email = find_email(document)?;
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

        if person.is_student == Some(true) {
            person.teacher_department = Some(find_department(&document)?);
        }

        Ok(person)
    }

    fn is_email_student(&self) -> bool {
        self.email.ends_with("@students.emmanuel.qld.edu.au")
    }

    fn is_email_teacher(&self) -> bool {
        self.email.ends_with("@emmanuel.qld.edu.au")
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

/// Will fail if not teachers
fn find_department(document: &scraper::Html) -> Result<String> {
    let selector =
        scraper::Selector::parse(r##"div.profile.content>div.row>div.columns>dl>dd:empty"##)
            .unwrap();
    let mut email = document.select(&selector);
    match email.next() {
        None => Err(color_eyre::eyre::eyre!("No email found")),
        // in general the department only appears for teachers,
        // and many similar fields exist that we ignore
        Some(email) => Ok(email.text().collect::<String>()),
    }
}
