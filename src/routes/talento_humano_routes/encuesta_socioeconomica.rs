use axum::{Json, Router, extract::State, routing::{get, post}};

use crate::{
    app::{app::AppState, error::ApiError, extractor::TokenData},
    controller::talento_humano::encuesta_socioeconomica::{encuesta_socioeconomica_controller, get_encuesta_socioeconomica_controller},
    models::talento_humano::personal::{EncuestaSocioeconomicaDto, Personal},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/talento-humano/encuesta-socioeconomica", post(encuesta_socioeconomica_endpoint))
        .route("/talento-humano/encuesta-socioeconomica", get(encuesta_socioeconomica_get_endpoint))
}

async fn encuesta_socioeconomica_endpoint(
    State(state): State<AppState>,
    auth: TokenData,
    Json(body): Json<EncuestaSocioeconomicaDto>,
) -> Result<(), ApiError> {
    encuesta_socioeconomica_controller(&state.db, auth.empleado_id, body).await
}

async fn encuesta_socioeconomica_get_endpoint(
    State(state): State<AppState>,
    token_data: TokenData,
) -> Result<Json<Personal>, ApiError> {
    let personal = get_encuesta_socioeconomica_controller(&state.db, token_data).await?;
    Ok(Json(personal))
}
