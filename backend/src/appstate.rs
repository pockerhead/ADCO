use once_cell::sync::Lazy;
use sqlx::postgres::{PgPool};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{OnceCell};
use tracing::info;

#[derive(Debug, Clone)]
pub struct AppState {
    pg_pool: OnceCell<Arc<PgPool>>,
}

pub static APP_STATE: Lazy<AppState> = Lazy::new(|| AppState::new());

impl AppState {
    pub fn new() -> Self {
        Self {
            pg_pool: OnceCell::const_new(),
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
