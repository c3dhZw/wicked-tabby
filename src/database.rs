use sqlx::PgPool;

pub struct Database {
  pub pool: PgPool,
}

impl Database {
  pub async fn new(target: &str) -> anyhow::Result<Self> {
    let pool = PgPool::connect(&format!("sqlite://{target}?mode=rwc")).await?;

    Ok(Self { pool })
  }
}
