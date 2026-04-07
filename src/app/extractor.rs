use axum::{extract::FromRequestParts, http::{header, request::Parts}};
use jsonwebtoken::{decode, DecodingKey, Validation};
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

impl <S> FromRequestParts<S> for TokenData 
where 
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
            parts: &mut Parts,
            _state: &S,
        ) -> Result<Self, Self::Rejection> {
        
        let cookie_header = parts.headers.get(header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let token = cookie_header.split(';')
            .find_map(|part| part.trim().strip_prefix("token="))
            .ok_or_else(|| ApiError::Unauthorized("No hay sesión activa".to_string()))?;

        let jwt_secret = std::env::var("JWT_SECRET")
            .map_err(|_| ApiError::InternalError("JWT_SECRET no configurado".to_string()))?;

        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| ApiError::Unauthorized("Token inválido o expirado".to_string()))?;

                let empleado_id = ObjectId::parse_str(&data.claims.sub)
            .map_err(|_| ApiError::InternalError("ID inválido en el token".to_string()))?;

        Ok(TokenData { empleado_id, pe: data.claims.pe })
    }
}