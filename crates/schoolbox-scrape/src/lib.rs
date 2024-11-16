pub use errors::{Error, Result};
use tracing::info;
mod errors {
    pub type Result<T> = color_eyre::eyre::Result<T>;
    pub type Error = color_eyre::eyre::Report;
}

#[derive(Debug)]
pub struct Person {
    name: String,
    email: String,
}

fn cookie() -> String {
    const COOKIE: &str = include_str!("cookie.txt");
    COOKIE.trim().into()
}

pub async fn get_website(client: &reqwest::Client, num: u32) -> Result<String> {
    info!(message = "Getting student", %num);
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

