use mongodb::bson::doc;

use crate::app::error::ApiError;
use crate::app::extractor::TokenData;
use crate::db::mongodb::MongoDb;
use crate::models::talento_humano::personal::Personal;
use crate::repository::talento_humano::personal::{PersonalRepository, QueryOptions};

pub async fn me(db: &MongoDb, token_data: TokenData) -> Result<Personal, ApiError> {
    let id = token_data.empleado_id;
    println!("Empleado ID extraído del token: {}", id);
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
