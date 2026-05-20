use axum::{routing::get, Router};

pub const ROUTES: &[(&str, &str, &str)] = &[("GET", "/health", "Check if the API is running")];

pub async fn health() -> &'static str {
    "OK"
}

pub fn router() -> Router {
    Router::new().route("/", get(health))
}
