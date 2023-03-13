use simple_logger::SimpleLogger;

pub mod base81;
pub mod errors;
pub mod files;
pub mod server;

fn main() -> anyhow::Result<()> {
  SimpleLogger::new().env().init()?;

  // @luug hi make this say something cool
  log::info!("Hello, world!");

  server::start("0.0.0.0:3000")?;

  Ok(())
}

// id      | clickies | url                | expires      | user-id
// --------|----------|--------------------|--------------|--------------
// gyfhujs | 2        | https://google.com | 1678417266   | awa-owo-uwu
