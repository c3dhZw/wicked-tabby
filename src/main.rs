use simple_logger::SimpleLogger;
use tab_kat::server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  SimpleLogger::new().env().init()?;

  // @luug hi make this say something cool
  log::info!("cheese burger wopper");

  server::start("0.0.0.0:8080").await?;

  Ok(())
}

// id      | clickies | url                | expires      | user-id
// --------|----------|--------------------|--------------|--------------
// gyfhujs | 2        | https://google.com | 1678417266   | awa-owo-uwu
