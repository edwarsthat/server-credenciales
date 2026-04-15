use axum::{Json, Router, extract::State, routing::{get, post}};

use crate::{
    app::{app::AppState, error::ApiError, extractor::{MaybeTokenData, TokenData}},
    controller::talento_humano::encuesta_socioeconomica::{encuesta_socioeconomica_controller, encuesta_socioeconomica_controller_without_token, get_encuesta_socioeconomica_controller},
    models::talento_humano::personal::{EncuestaSocioeconomicaDto, Personal},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/talento-humano/encuesta-socioeconomica", post(encuesta_socioeconomica_endpoint))
        .route("/talento-humano/encuesta-socioeconomica", get(encuesta_socioeconomica_get_endpoint))
}

async fn encuesta_socioeconomica_endpoint(
    State(state): State<AppState>,
    MaybeTokenData(maybe_token): MaybeTokenData,
    Json(body): Json<EncuestaSocioeconomicaDto>,
) -> Result<(), ApiError> {
    match maybe_token {
        Some(token_data) => {
            encuesta_socioeconomica_controller(&state.db, token_data.empleado_id, body).await
        }
        None => {
            encuesta_socioeconomica_controller_without_token(&state.redis, body).await
        }
    }
}

async fn encuesta_socioeconomica_get_endpoint(
    State(state): State<AppState>,
    token_data: TokenData,
) -> Result<Json<Personal>, ApiError> {
    let personal = get_encuesta_socioeconomica_controller(&state.db, token_data).await?;
    Ok(Json(personal))
}
