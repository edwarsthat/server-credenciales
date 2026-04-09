use mongodb::bson::{DateTime, doc, oid::ObjectId, to_document};

use crate::{
    app::error::ApiError,
    db::mongodb::MongoDb,
    models::talento_humano::personal::EncuestaSocioeconomicaDto,
    repository::talento_humano::personal::{PersonalRepository, UpdateOptions},
};

pub async fn encuesta_socioeconomica_controller(
    db: &MongoDb,
    empleado_id: ObjectId,
    body: EncuestaSocioeconomicaDto,
) -> Result<(), ApiError> {
    let mut set = to_document(&body)
        .map_err(|e| ApiError::InternalError(format!("Error al serializar datos: {}", e)))?;

    if set.is_empty() {
        return Err(ApiError::BadRequest("No se enviaron campos para actualizar".to_string()));
    }

    set.insert("fecha_formulario_sociodemografico", DateTime::now());

    let repo = PersonalRepository::new(&db.db);

    repo.patch_data(UpdateOptions {
        filter: Some(doc! { "_id": empleado_id }),
        update: Some(doc! { "$set": set }),
    })
    .await
    .map_err(|e| ApiError::InternalError(e.message().to_string()))
}
