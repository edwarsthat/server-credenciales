use axum::{Json, extract::State, routing::post, Router};

use crate::{
    app::{app::AppState, error::ApiError, extractor::TokenData},
    controller::talento_humano::encuesta_socioeconomica::encuesta_socioeconomica_controller,
    models::talento_humano::personal::EncuestaSocioeconomicaDto,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/talento-humano/encuesta-socioeconomica", post(encuesta_socioeconomica_endpoint))
}

async fn encuesta_socioeconomica_endpoint(
    State(state): State<AppState>,
    auth: TokenData,
    Json(body): Json<EncuestaSocioeconomicaDto>,
) -> Result<(), ApiError> {
    encuesta_socioeconomica_controller(&state.db, auth.empleado_id, body).await
}
