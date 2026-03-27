use jsonwebtoken::{EncodingKey, Header, encode};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::app::error::ApiError;
use crate::db::mongodb::MongoDb;
use crate::models::talento_humano::personal::Personal;
use crate::repository::talento_humano::carnets::{CarnetsRepository, QueryOptions as CarnetQueryOptions};
use crate::repository::talento_humano::personal::{PersonalRepository, QueryOptions as PersonalQueryOptions};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    pe: i64,
    exp: u64,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub personal: Personal,
    #[serde(skip_serializing)]
    pub token: String,
}

pub async fn verify_carnet(db: &MongoDb, serial: i64, token: String) -> Result<VerifyResponse, ApiError> {
    let carnets_repo = CarnetsRepository::new(&db.db);

    let mut results = carnets_repo
        .get_data(CarnetQueryOptions {
            filter: Some(doc! { "SKU": serial as i32 }),
            populate: false,
            ..Default::default()
        })
        .await
        .map_err(|e| ApiError::InternalError(e.message().to_string()))?;

    let carnet = results
        .pop()
        .ok_or_else(|| ApiError::NotFound(format!("Carnet con serial {} no encontrado", serial)))?;

    let hash = carnet
        .token_hash
        .as_deref()
        .ok_or_else(|| ApiError::Unauthorized("Este carnet no tiene token asignado".to_string()))?;

    let token_valido = bcrypt::verify(&token, hash)
        .map_err(|_| ApiError::InternalError("Error al verificar el token".to_string()))?;

    if !token_valido {
        return Err(ApiError::Unauthorized("Token inválido".to_string()));
    }

    let employee_id = carnet.employee_id
        .ok_or_else(|| ApiError::NotFound("El carnet no tiene empleado asignado".to_string()))?;

    let personal_repo = PersonalRepository::new(&db.db);

    let mut personal_results = personal_repo
        .get_data(PersonalQueryOptions {
            filter: Some(doc! { "_id": employee_id }),
            populate: true,
            ..Default::default()
        })
        .await
        .map_err(|e| ApiError::InternalError(e.message().to_string()))?;

    let personal = personal_results
        .pop()
        .ok_or_else(|| ApiError::NotFound("Empleado no encontrado".to_string()))?;

    if !personal.estado {
        return Err(ApiError::Unauthorized("El empleado está inactivo".to_string()));
    }

    let jwt_secret = std::env::var("JWT_SECRET")
        .map_err(|_| ApiError::InternalError("JWT_SECRET no configurado".to_string()))?;

    let expiracion = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() + 3600; // 1 hora

    let claims = Claims {
        sub: personal.id.map(|id| id.to_hex()).unwrap_or_default(),
        pe: personal.pe,
        exp: expiracion,
    };

    let jwt = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| ApiError::InternalError("Error generando el token JWT".to_string()))?;

    Ok(VerifyResponse {
        personal,
        token: jwt,
    })
}
