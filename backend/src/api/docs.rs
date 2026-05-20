use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};

use super::health;

pub async fn openapi() -> Json<Value> {
    let routes = health::ROUTES
        .iter()
        .map(|(method, path, description)| {
            json!({
                "method": method,
                "path": path,
                "description": description
            })
        })
        .collect::<Vec<_>>();

    Json(json!({
        "name": "SortMe API",
        "version": "0.1.0",
        "routes": routes
    }))
}

pub fn router() -> Router {
    Router::new().route("/", get(openapi))
}
