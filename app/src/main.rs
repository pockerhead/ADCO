mod appstate;
mod domain;

use domain::orchestrator::Orchestrator;
use dotenvy::dotenv;
use tokio::spawn;
use tracing::info;
use tracing_subscriber;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use appstate::APP_STATE;

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
        _ = APP_STATE.get_pg_pool().await;
        info!("Database connected");
    });

    if APP_STATE.is_interactive_mode {
        info!("Interactive mode");
        info!("Orchestrator starting");
        let orchestrator = Orchestrator {};
        orchestrator.orchestrate().await?;
    } else {
        info!("Non-interactive mode");
        let server_handle = tokio::spawn(async {
            match start_server().await {
                Ok(_) => info!("never reached"),
                Err(e) => info!("Server error: {}", e),
            }
        });
        match server_handle.await {
            Ok(_) => info!("Never reached"),
            Err(e) => info!("Server error: {}", e),
        }
    }

    return Ok(());
}

async fn start_server() -> anyhow::Result<()> {
    let app = Router::new().route("/health", get(|| async { "Healthy!!!" }));
    let listener = TcpListener::bind("127.0.0.1:8084").await?;
    info!("Server started");
    axum::serve(listener, app).await?;
    Ok(())
}