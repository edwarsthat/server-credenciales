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
            projection: Some(doc! {
                "id": 1,
                "PE": 1,
                "nombre": 1,
                "cargo": 1,
                "identificacion": 1,
                "tipo_documento": 1,
                "foto": 1,
                "tipo_sangre": 1,
                "url_foto_carnet": 1,
                "estado": 1,
                "fecha_formulario_sociodemografico": 1
            }),
            populate: true,
            ..Default::default()
        })
        .await
        .map_err(|e| ApiError::InternalError(e.message().to_string()))?;

    results
        .pop()
        .ok_or_else(|| ApiError::NotFound("Empleado no encontrado".to_string()))
}
