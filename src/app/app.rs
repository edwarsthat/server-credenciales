use axum::Router;
use axum::http::{HeaderValue, Method, header};
use axum::middleware;
use tower_http::cors::CorsLayer;

use crate::app::error::ApiError;
use crate::app::rate_limiter::{RateLimiter, rate_limit_middleware};
use crate::db::mongodb::MongoDb;
use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub db: MongoDb,
}

pub fn create_router(db_pool: MongoDb) -> Router {
    let state = AppState { db: db_pool };

    let cors = CorsLayer::new()
        .allow_origin([
            "https://credenciales.celifrut.com".parse::<HeaderValue>().unwrap(),
            "http://localhost:5174".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .allow_credentials(true);

    // 60 peticiones por minuto por IP
    let limiter = RateLimiter::new(60, 60);

    Router::new()
        .merge(routes::auth::routes())
        .merge(routes::media::routes())
        .merge(routes::healt::routes())
        .merge(routes::talento_humano::routes())
        .merge(routes::talento_humano_routes::encuesta_socioeconomica::routes())
        .fallback(handler_404)
        .layer(middleware::from_fn_with_state(limiter, rate_limit_middleware))
        .layer(cors)
        .with_state(state)
}

async fn handler_404() -> ApiError {
    ApiError::NotFound("Ruta no encontrada".to_string())
}