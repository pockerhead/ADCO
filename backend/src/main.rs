mod domain;

use domain::orchestrator::Orchestrator;
use dotenvy::dotenv;
use tokio::time::Duration;
use std::time::Instant;
use tracing_subscriber;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("adco_backend=info".parse()?),
        )
        .with_target(false)
        .init();
    let start_time = Instant::now();
    let database_url = std::env::var("DATABASE_URL")?;
    let options: sqlx::pool::PoolOptions<sqlx::postgres::Postgres> = sqlx::pool::PoolOptions::new()
        .max_connections(2)
        .min_connections(0);
    let pg_pool = options.connect(&database_url).await?;
    info!("Database connected in {:?}", start_time.elapsed());
    let orchestrator = Orchestrator::new(pg_pool.clone());
    orchestrator.orchestrate().await?;
    info!("Orchestrator completed in {:?}", start_time.elapsed());
    return Ok(());
}
