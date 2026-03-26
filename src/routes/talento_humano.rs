use axum::{Json, Router, extract::State, routing::post};
use serde::Deserialize;

use crate::app::app::AppState;
use crate::app::error::ApiError;
use crate::controller::talento_humano::personal::{VerifyResponse, verify_carnet};

#[derive(Deserialize, Debug)]
pub struct VerifyBody {
    pub serial: i64,
    pub token: String,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/talento_humano/verify", post(verify))
}

async fn verify(
    State(state): State<AppState>,
    Json(body): Json<VerifyBody>,
) -> Result<Json<VerifyResponse>, ApiError> {
    println!("{:?}", body);
    let carnet = verify_carnet(&state.db, body.serial, body.token).await?;
    Ok(Json(carnet))
}
