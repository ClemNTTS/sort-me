use std::net::SocketAddr;

use axum::Router;
use tower_http::trace::TraceLayer;
use tracing::info;

pub mod api;
pub mod db;
pub mod env_manager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_manager::load_env();

    tracing_subscriber::fmt()
        .with_env_filter(env_manager::get_rust_log())
        .init();

    let database_url = env_manager::get_database_url();
    let db_pool = db::connect(&database_url).await?;
    db::run_migrations(&db_pool).await?;
    db::health_check(&db_pool).await?;

    let app: Router<_> = Router::new()
        .merge(api::router())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], env_manager::get_app_port()));
    info!("SortMe backend running on : http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
