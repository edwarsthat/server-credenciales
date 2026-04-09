use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::Serialize;

use crate::app::app::AppState;
use crate::controller::healt::check_health;

#[derive(Serialize)]
struct HealthResponse {
    status: bool,
    message: String,
    db: DbStatus,
}

#[derive(Serialize)]
struct DbStatus {
    ok: bool,
    message: String,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    let health = check_health(&state.db).await;

    let status_code = if health.db_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    let body = Json(HealthResponse {
        status: health.db_ok,
        message: if health.db_ok {
            "Server is running".to_string()
        } else {
            "Service degraded".to_string()
        },
        db: DbStatus {
            ok: health.db_ok,
            message: if health.db_ok {
                "Database available".to_string()
            } else {
                "Database unavailable".to_string()
            },
        },
    });

    (status_code, body).into_response()
}
