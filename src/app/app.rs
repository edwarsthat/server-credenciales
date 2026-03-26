use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use crate::app::error::ApiError;
use crate::db::mongodb::MongoDb;
use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub db: MongoDb,
}

pub fn create_router(db_pool: MongoDb) -> Router {
    let state = AppState { db: db_pool };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(routes::healt::routes())
        .merge(routes::talento_humano::routes())
        .fallback(handler_404)
        .layer(cors)
        .with_state(state)
}

async fn handler_404() -> ApiError {
    ApiError::NotFound("Ruta no encontrada".to_string())
}