use axum::{extract::State, http::StatusCode, response::Json, routing::get, Router};
use serde::Serialize;

use crate::{db, state::AppState};

pub const ROUTES: &[(&str, &str, &str)] = &[(
    "GET",
    "/health",
    "Check if the API and database are running",
)];

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub database: &'static str,
}

pub async fn health(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, (StatusCode, Json<HealthResponse>)> {
    if db::health_check(&state.db_pool).await.is_err() {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthResponse {
                status: "degraded",
                database: "unavailable",
            }),
        ));
    }

    Ok(Json(HealthResponse {
        status: "ok",
        database: "ok",
    }))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(health))
}
