use axum::Router;

use crate::app::error::ApiError;
use crate::db::mongodb::MongoDb;
use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub db: MongoDb,
}

pub fn create_router(db_pool: MongoDb) -> Router {
    let state = AppState { db: db_pool };

    Router::new()
        .merge(routes::healt::routes())
        .fallback(handler_404)
        .with_state(state)
}

async fn handler_404() -> ApiError {
    ApiError::NotFound("Ruta no encontrada".to_string())
}