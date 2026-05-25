use std::net::SocketAddr;

use axum::Router;
use tower_http::trace::TraceLayer;
use tracing::info;

use config::{env, AppConfig};
use state::AppState;

pub mod api;
pub mod classifier;
pub mod config;
pub mod db;
pub mod llm;
pub mod mail;
pub mod scheduler;
pub mod security;
pub mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::load_env();

    tracing_subscriber::fmt()
        .with_env_filter(env::get_rust_log())
        .init();

    let config = AppConfig::from_env();
    let db_pool = db::connect(&config.database_url).await?;
    db::run_migrations(&db_pool).await?;
    db::health_check(&db_pool).await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], config.app_port));
    let state = AppState { config, db_pool };

    let app: Router<_> = Router::new()
        .merge(api::router())
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    info!("SortMe backend running on : http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
