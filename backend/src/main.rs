mod appstate;
mod domain;

use appstate::{AppState, APP_STATE};
use axum::{routing::get, Router};
use domain::orchestrator::Orchestrator;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    AppState::initialize()?;

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
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await?;
    Ok(())
}
