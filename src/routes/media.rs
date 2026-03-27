use axum::{Router, extract::{Path, State}, http::header, response::IntoResponse, routing::get};
use axum::http::{HeaderMap, HeaderValue};

use crate::app::app::AppState;
use crate::app::error::ApiError;
use crate::controller::media::get_foto;

pub fn routes() -> Router<AppState> {
    Router::new().route("/media/foto/{*filename}", get(handler_foto))
}

async fn handler_foto(
    State(_state): State<AppState>,
    Path(filename): Path<String>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let cookie_header = headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let token = cookie_header
        .split(';')
        .find_map(|part| part.trim().strip_prefix("token="))
        .ok_or_else(|| ApiError::Unauthorized("No hay sesión activa".to_string()))?;

    let (bytes, content_type) = get_foto(token, &filename).await?;

    let mut response_headers = HeaderMap::new();
    response_headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));

    Ok((response_headers, bytes))
}
