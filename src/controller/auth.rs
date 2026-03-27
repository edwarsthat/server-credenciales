use jsonwebtoken::{DecodingKey, Validation, decode};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::app::error::ApiError;
use crate::db::mongodb::MongoDb;
use crate::models::talento_humano::personal::Personal;
use crate::repository::talento_humano::personal::{PersonalRepository, QueryOptions};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    pe: i64,
    exp: u64,
}

pub fn validate_token(token: &str) -> Result<(), ApiError> {
    let jwt_secret = std::env::var("JWT_SECRET")
        .map_err(|_| ApiError::InternalError("JWT_SECRET no configurado".to_string()))?;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| ApiError::Unauthorized("Token inválido o expirado".to_string()))?;

    Ok(())
}

pub async fn me(db: &MongoDb, token: &str) -> Result<Personal, ApiError> {
    let jwt_secret = std::env::var("JWT_SECRET")
        .map_err(|_| ApiError::InternalError("JWT_SECRET no configurado".to_string()))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| ApiError::Unauthorized("Token inválido o expirado".to_string()))?;

    let id = ObjectId::parse_str(&token_data.claims.sub)
        .map_err(|_| ApiError::InternalError("ID inválido en el token".to_string()))?;

    let personal_repo = PersonalRepository::new(&db.db);

    let mut results = personal_repo
        .get_data(QueryOptions {
            filter: Some(doc! { "_id": id }),
            populate: true,
            ..Default::default()
        })
        .await
        .map_err(|e| ApiError::InternalError(e.message().to_string()))?;

    results
        .pop()
        .ok_or_else(|| ApiError::NotFound("Empleado no encontrado".to_string()))
}
