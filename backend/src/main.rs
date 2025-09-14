mod domain;

use domain::orchestrator::Orchestrator;
use dotenvy::dotenv;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let mut options: sqlx::pool::PoolOptions<sqlx::postgres::Postgres> = sqlx::pool::PoolOptions::new();
    let pg_pool = options.connect(&database_url).await?;
    let orchestrator = Orchestrator::new(pg_pool.clone());
    orchestrator.orchestrate().await?;
    return Ok(())
}



