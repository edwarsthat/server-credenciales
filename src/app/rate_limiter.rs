use std::net::{IpAddr, SocketAddr};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use axum::{
    body::Body,
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use moka::sync::Cache;

#[derive(Clone)]
pub struct RateLimiter {
    cache: Arc<Cache<IpAddr, Arc<AtomicU32>>>,
    max_requests: u32,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_secs: u64) -> Self {
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(window_secs))
            .build();

        Self {
            cache: Arc::new(cache),
            max_requests,
        }
    }

    fn is_allowed(&self, ip: IpAddr) -> bool {
        // or_insert_with es atómico: si la entrada no existe la crea, si existe retorna la actual
        let counter = self
            .cache
            .entry(ip)
            .or_insert_with(|| Arc::new(AtomicU32::new(0)));

        // fetch_add retorna el valor ANTES de incrementar
        let prev = counter.value().fetch_add(1, Ordering::Relaxed);
        prev < self.max_requests
    }
}

pub async fn rate_limit_middleware(
    State(limiter): State<RateLimiter>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse::<IpAddr>().ok())
        .unwrap_or(addr.ip());

    if !limiter.is_allowed(ip) {
        return Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .header("Retry-After", "60")
            .body(Body::from("Too many requests"))
            .unwrap();
    }

    next.run(req).await
}
