use actix_web::{web, HttpResponse};
use shared_lib::Result;
use std::sync::Arc;
use std::time::Instant;

use crate::models::{HealthResponse, ReadyResponse};
use crate::services::FaviconService;

lazy_static::lazy_static! {
    static ref START_TIME: Instant = Instant::now();
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 503, description = "Service is unhealthy")
    ),
    tag = "Health"
)]
pub async fn health(favicon_service: web::Data<Arc<FaviconService>>) -> Result<HttpResponse> {
    let uptime = START_TIME.elapsed().as_secs();
    let (redis_connected, redis_latency) = favicon_service.check_redis_health().await;

    let response = HealthResponse {
        status: if redis_connected {
            "healthy".to_string()
        } else {
            "degraded".to_string()
        },
        service: "utility-service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime,
        redis: crate::models::RedisHealth {
            connected: redis_connected,
            latency_ms: redis_latency,
        },
    };

    let status = if redis_connected { 200 } else { 503 };

    Ok(HttpResponse::build(actix_web::http::StatusCode::from_u16(status).unwrap()).json(response))
}

/// Readiness check endpoint
#[utoipa::path(
    get,
    path = "/api/v1/ready",
    responses(
        (status = 200, description = "Service is ready", body = ReadyResponse),
        (status = 503, description = "Service is not ready")
    ),
    tag = "Health"
)]
pub async fn ready(favicon_service: web::Data<Arc<FaviconService>>) -> Result<HttpResponse> {
    let (redis_connected, _) = favicon_service.check_redis_health().await;

    let response = ReadyResponse {
        ready: redis_connected,
        service: "utility-service".to_string(),
    };

    let status = if redis_connected { 200 } else { 503 };

    Ok(HttpResponse::build(actix_web::http::StatusCode::from_u16(status).unwrap()).json(response))
}

/// Prometheus metrics endpoint
#[utoipa::path(
    get,
    path = "/api/v1/metrics",
    responses(
        (status = 200, description = "Prometheus metrics", content_type = "text/plain")
    ),
    tag = "Health"
)]
pub async fn metrics() -> Result<HttpResponse> {
    let uptime = START_TIME.elapsed().as_secs();

    let metrics = format!(
        r#"# HELP utility_service_info Service information
# TYPE utility_service_info gauge
utility_service_info{{version="{}"}} 1

# HELP utility_service_uptime_seconds Service uptime in seconds
# TYPE utility_service_uptime_seconds counter
utility_service_uptime_seconds {}
"#,
        env!("CARGO_PKG_VERSION"),
        uptime
    );

    Ok(HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(metrics))
}
