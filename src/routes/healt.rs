use axum::{Json, Router, routing::get};
use serde::Serialize;

use crate::app::app::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: bool,
    message: String,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: true,
        message: "Server is running".to_string(),
    })
}
