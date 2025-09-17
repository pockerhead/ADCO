use once_cell::sync::Lazy;
use sqlx::postgres::{PgPool};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{OnceCell};
use tracing::info;
use tracing_subscriber;
use dotenvy::dotenv;
use tokio::spawn;

#[derive(Debug, Clone)]
pub struct AppState {
    pg_pool: OnceCell<Arc<PgPool>>,
    pub is_interactive_mode: bool,
}

pub static APP_STATE: Lazy<AppState> = Lazy::new(|| AppState::new());

impl AppState {

    pub fn initialize() -> anyhow::Result<()> {
        dotenv().ok();
        // Initialize logging
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive("adco_backend=info".parse()?),
            )
            .with_target(false)
            .init();
    
        info!("Starting ADCO backend");
        info!("Database connecting");
        info!(
            "anthropic api key: {}",
            std::env::var("ANTHROPIC_API_KEY").unwrap_or_default()
        );
    
        spawn(async {
            info!("Database started connecting");
            _ = APP_STATE.get_pg_pool().await;
            info!("Database connected");
        });

        Ok(())
    }

    pub fn new() -> Self {
        let is_interactive_mode = std::env::var("IS_INTERACTIVE_MODE")
        .unwrap_or("false".to_string())
        .parse::<bool>()
        .unwrap_or(false);
        Self {
            pg_pool: OnceCell::const_new(),
            is_interactive_mode: is_interactive_mode,
        }
    }

    pub async fn get_pg_pool(&self) -> Arc<PgPool> {
        info!("Getting pg pool");
        let start_time = Instant::now();
        self.pg_pool
            .get_or_init(|| async move {
                info!("Initializing pg pool");
                let database_url = std::env::var("DATABASE_URL").unwrap();
                let options: sqlx::pool::PoolOptions<sqlx::postgres::Postgres> =
                    sqlx::pool::PoolOptions::new()
                        .max_connections(2)
                        .min_connections(0);
                info!("Connecting to database");
                let pg_pool = options.connect(&database_url).await.unwrap();
                info!("Database connected");
                info!("Database connected in {:?}", start_time.elapsed());
                Arc::new(pg_pool)
            })
            .await
            .clone()
    }
}
