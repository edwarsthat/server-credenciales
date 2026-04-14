use axum::{Json, Router, extract::State, routing::get};

use crate::app::app::AppState;
use crate::app::error::ApiError;
use crate::app::extractor::TokenData;
use crate::controller::auth::me;
use crate::models::talento_humano::personal::Personal;

pub fn routes() -> Router<AppState> {
    Router::new().route("/auth/me", get(handler_me))
}

async fn handler_me(
    State(state): State<AppState>,
    token_data: TokenData,
) -> Result<Json<Personal>, ApiError> {
    let personal = me(&state.db, token_data).await?;
    Ok(Json(personal))
}
