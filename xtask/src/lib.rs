use openssh::{KnownHosts, Session};

pub mod cli;

pub type Error = color_eyre::Report;
pub type Result<T> = color_eyre::Result<T>;

pub async fn connect_to_server() -> Result<openssh::Session> {
    Ok(Session::connect("ymap", KnownHosts::Strict).await?)
}