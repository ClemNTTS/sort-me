use axum::Router;

pub mod docs;
pub mod health;

pub fn router() -> Router {
    Router::new()
        .nest("/health", health::router())
        .nest("/docs", docs::router())
}
