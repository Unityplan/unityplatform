use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::time::Instant;
use tracing::{info, warn};

use super::request_id::get_request_id;
use crate::metrics::MetricsCollector;

/// Logging middleware with structured logging
///
/// Logs all requests with:
/// - Request ID
/// - HTTP method
/// - Path
/// - Status code
/// - Duration
/// - User agent (Phase 1 only)
///
/// # Phase 1 (Development)
/// - DEBUG level enabled
/// - Logs user agent and referer
/// - Verbose query params
///
/// # Phase 2 (Production)
/// - INFO/WARN/ERROR only
/// - No user agent or referer (privacy)
/// - Redacted query params
///
/// # Example
///
/// ```rust
/// use actix_web::{web, App, HttpServer};
/// use shared_lib::middleware::LoggingMiddleware;
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     HttpServer::new(|| {
///         App::new()
///             .wrap(LoggingMiddleware::new(false)) // false = production
///             // ... routes
///     })
///     .bind(("127.0.0.1", 8080))?
///     .run()
///     .await
/// }
/// ```
#[derive(Debug, Clone)]
pub struct LoggingMiddleware {
    /// Enable verbose logging (Phase 1 = true, Phase 2 = false)
    verbose: bool,
    /// Optional metrics collector
    metrics: Option<MetricsCollector>,
}

impl LoggingMiddleware {
    /// Create new logging middleware
    ///
    /// # Arguments
    /// * `verbose` - true for development, false for production
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            metrics: None,
        }
    }

    /// Create logging middleware with metrics collection
    pub fn with_metrics(verbose: bool, metrics: MetricsCollector) -> Self {
        Self {
            verbose,
            metrics: Some(metrics),
        }
    }

    /// Development logging - verbose with detailed request information
    pub fn development() -> Self {
        Self {
            verbose: true,
            metrics: None,
        }
    }

    /// Development logging with metrics
    pub fn development_with_metrics(metrics: MetricsCollector) -> Self {
        Self {
            verbose: true,
            metrics: Some(metrics),
        }
    }

    /// Production logging - minimal, only essential information
    pub fn production() -> Self {
        Self {
            verbose: false,
            metrics: None,
        }
    }

    /// Production logging with metrics
    pub fn production_with_metrics(metrics: MetricsCollector) -> Self {
        Self {
            verbose: false,
            metrics: Some(metrics),
        }
    }

    /// Alias for development() - kept for backward compatibility
    #[deprecated(since = "0.1.0-alpha.1", note = "Use development() instead")]
    pub fn phase1() -> Self {
        Self::development()
    }

    /// Alias for production() - kept for backward compatibility
    #[deprecated(since = "0.1.0-alpha.1", note = "Use production() instead")]
    pub fn phase2() -> Self {
        Self::production()
    }
}

impl Default for LoggingMiddleware {
    fn default() -> Self {
        // Default to production (Phase 2) for safety
        Self {
            verbose: false,
            metrics: None,
        }
    }
}

// Middleware factory
impl<S, B> Transform<S, ServiceRequest> for LoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddlewareService {
            service,
            verbose: self.verbose,
            metrics: self.metrics.clone(),
        }))
    }
}

pub struct LoggingMiddlewareService<S> {
    service: S,
    verbose: bool,
    metrics: Option<MetricsCollector>,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let method = req.method().to_string();
        let path = req.path().to_string();
        let query = req.query_string().to_string();
        let request_id = get_request_id(req.request()).unwrap_or_else(|| "unknown".to_string());
        let verbose = self.verbose;
        let metrics = self.metrics.clone();

        // Phase 1: Log user agent
        let user_agent = if verbose {
            req.headers()
                .get("user-agent")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string())
        } else {
            None
        };

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let status = res.status();
            let duration = start.elapsed();

            // Record metrics if collector is available
            if let Some(ref collector) = metrics {
                collector.record_request(&method, &path, status.as_u16(), duration.as_secs_f64());
            }

            // Log based on status code
            if status.is_server_error() {
                // 5xx errors = ERROR level
                if verbose {
                    tracing::error!(
                        request_id = %request_id,
                        method = %method,
                        path = %path,
                        query = %query,
                        status = %status.as_u16(),
                        duration_ms = %duration.as_millis(),
                        user_agent = ?user_agent,
                        "Request failed with server error"
                    );
                } else {
                    tracing::error!(
                        request_id = %request_id,
                        method = %method,
                        path = %path,
                        status = %status.as_u16(),
                        duration_ms = %duration.as_millis(),
                        "Request failed with server error"
                    );
                }
            } else if status.is_client_error() {
                // 4xx errors = WARN level
                if verbose {
                    warn!(
                        request_id = %request_id,
                        method = %method,
                        path = %path,
                        query = %query,
                        status = %status.as_u16(),
                        duration_ms = %duration.as_millis(),
                        user_agent = ?user_agent,
                        "Request failed with client error"
                    );
                } else {
                    warn!(
                        request_id = %request_id,
                        method = %method,
                        path = %path,
                        status = %status.as_u16(),
                        duration_ms = %duration.as_millis(),
                        "Request failed with client error"
                    );
                }
            } else {
                // 2xx/3xx = INFO level
                if verbose {
                    info!(
                        request_id = %request_id,
                        method = %method,
                        path = %path,
                        query = %query,
                        status = %status.as_u16(),
                        duration_ms = %duration.as_millis(),
                        user_agent = ?user_agent,
                        "Request completed"
                    );
                } else {
                    info!(
                        request_id = %request_id,
                        method = %method,
                        path = %path,
                        status = %status.as_u16(),
                        duration_ms = %duration.as_millis(),
                        "Request completed"
                    );
                }
            }

            Ok(res)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App, HttpResponse};

    #[actix_web::test]
    async fn test_logging_middleware() {
        let app = test::init_service(App::new().wrap(LoggingMiddleware::development()).route(
            "/test",
            web::get().to(|| async { HttpResponse::Ok().finish() }),
        ))
        .await;

        let req = test::TestRequest::get()
            .uri("/test?foo=bar")
            .insert_header(("user-agent", "test-agent"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
