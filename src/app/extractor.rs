use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::app::error::ApiError;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    pe: i64,
    exp: u64,
}

pub struct TokenData {
    pub empleado_id: ObjectId,
    pub pe: i64,
}

impl<S> FromRequestParts<S> for TokenData
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookie_header = parts
            .headers
            .get(header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let token = cookie_header
            .split(';')
            .find_map(|part| part.trim().strip_prefix("token="))
            .ok_or_else(|| ApiError::Unauthorized("No hay sesión activa".to_string()))?;

        // Modo desarrollo: si DEV_TOKEN está definido y coincide, retorna un TokenData fijo
        if let Ok(dev_token) = std::env::var("DEV_TOKEN") {
            if token == dev_token {
                return Ok(TokenData {
                    empleado_id: ObjectId::parse_str("69d3d516279d8dcac927c15c").unwrap(),
                    pe: 139,
                });
            }
        }

        let jwt_secret = std::env::var("JWT_SECRET")
            .map_err(|_| ApiError::InternalError("JWT_SECRET no configurado".to_string()))?;

        let mut validation = Validation::default();
        validation.validate_exp = true;

        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &validation,
        )
        .map_err(|_| ApiError::Unauthorized("Token inválido o expirado".to_string()))?;

        let empleado_id = ObjectId::parse_str(&data.claims.sub)
            .map_err(|_| ApiError::InternalError("ID inválido en el token".to_string()))?;

        Ok(TokenData {
            empleado_id,
            pe: data.claims.pe,
        })
    }
}
