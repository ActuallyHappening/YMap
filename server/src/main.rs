#[tokio::main]
async fn main() -> Result<(), impl std::fmt::Debug> {
  utils::tracing::install_tracing("info,server=trace,app=trace,db=debug,payments=debug")?;

  let prod = cfg!(feature = "prod");
  tracing::info!(prod, "Logging started on the server");

  server::main().await
}
