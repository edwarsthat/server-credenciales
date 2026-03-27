use axum::{Json, Router, extract::State, http::header, response::IntoResponse, routing::{get, post}};
use axum::http::HeaderMap;
use serde::Deserialize;

use crate::app::app::AppState;
use crate::app::error::ApiError;
use crate::controller::talento_humano::personal::verify_carnet;

#[derive(Deserialize, Debug)]
pub struct VerifyBody {
    pub serial: i64,
    pub token: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/talento_humano/verify", post(verify))
        .route("/talento_humano/areas_acceso", get(areas_acceso))
}

async fn verify(
    State(state): State<AppState>,
    Json(body): Json<VerifyBody>,
) -> Result<impl IntoResponse, ApiError> {
    println!("{:?}", body);
    let carnet = verify_carnet(&state.db, body.serial, body.token).await?;

    let cookie = format!(
        "token={}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=3600",
        carnet.token
    );

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    Ok((headers, Json(carnet)))
}


async fn areas_acceso(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::models::instalaciones::AreaFisica>>, ApiError> {
    let areas = crate::controller::talento_humano::instalaciones::get_areas_acceso(&state.db).await?;
    Ok(Json(areas))
}