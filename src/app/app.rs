use axum::Router;
use axum::http::{HeaderValue, Method, header};
use tower_http::cors::CorsLayer;

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
        .allow_origin("https://credenciales.celifrut.com".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .allow_credentials(true);

    Router::new()
        .merge(routes::auth::routes())
        .merge(routes::media::routes())
        .merge(routes::healt::routes())
        .merge(routes::talento_humano::routes())
        .fallback(handler_404)
        .layer(cors)
        .with_state(state)
}

async fn handler_404() -> ApiError {
    ApiError::NotFound("Ruta no encontrada".to_string())
}