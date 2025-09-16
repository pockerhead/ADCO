mod appstate;
mod domain;

use domain::orchestrator::Orchestrator;
use dotenvy::dotenv;
use tokio::spawn;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    info!("Starting ADCO backend");
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("adco_backend=info".parse()?),
        )
        .with_target(false)
        .init();
    info!("Database connecting");
    spawn(async {
        info!("Database started connecting");
        _ = appstate::APP_STATE.get_pg_pool().await;
        info!("Database connected");
    });
    info!("Orchestrator starting");
    let orchestrator = Orchestrator {};
    orchestrator.orchestrate().await?;
    return Ok(());
}
