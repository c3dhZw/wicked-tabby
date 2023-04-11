use sqlx::SqlitePool;

pub struct Database {
  pub pool: SqlitePool,
}

impl Database {
  pub async fn new(target: &str) -> anyhow::Result<Self> {
    let pool = SqlitePool::connect(&format!("sqlite://{target}?mode=rwc")).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(Self { pool })
  }
}
