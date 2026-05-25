use axum::Router;

use crate::state::AppState;

pub mod docs;
pub mod health;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/health", health::router())
        .nest("/docs", docs::router())
}
