use crate::app::error::ApiError;
use crate::db::mongodb::MongoDb;
use crate::models::instalaciones::AreaFisica;
use crate::repository::instalaciones::{InstalacionesRepository, QueryOptions};

pub async fn get_areas_acceso(db: &MongoDb) -> Result<Vec<AreaFisica>, ApiError> {
    let repo = InstalacionesRepository::new(&db.db);
    repo.get_data(QueryOptions::default()).await
        .map_err(|e| ApiError::InternalError(e.message().to_string()))
}
