use axum::{Json, Router, extract::State, http::header, routing::get};
use axum::http::HeaderMap;

use crate::app::app::AppState;
use crate::app::error::ApiError;
use crate::controller::auth::me;
use crate::models::talento_humano::personal::Personal;

pub fn routes() -> Router<AppState> {
    Router::new().route("/auth/me", get(handler_me))
}

async fn handler_me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Personal>, ApiError> {
    let cookie_header = headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let token = cookie_header
        .split(';')
        .find_map(|part| part.trim().strip_prefix("token="))
        .ok_or_else(|| ApiError::Unauthorized("No hay sesión activa".to_string()))?;

    let personal = me(&state.db, token).await?;
    Ok(Json(personal))
}
