use axum::{Router, extract::Path, http::header, response::IntoResponse, routing::get};
use axum::http::{HeaderMap, HeaderValue};

use crate::app::app::AppState;
use crate::app::error::ApiError;
use crate::app::extractor::TokenData;
use crate::controller::media::get_foto;

pub fn routes() -> Router<AppState> {
    Router::new().route("/media/foto/{*filename}", get(handler_foto))
}

async fn handler_foto(
    _token_data: TokenData,
    Path(filename): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let (bytes, content_type) = get_foto(&filename).await?;

    let mut response_headers = HeaderMap::new();
    response_headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));

    Ok((response_headers, bytes))
}
