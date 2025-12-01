use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::HeaderValue,
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use uuid::Uuid;

/// Request ID for tracking requests across services
///
/// Extracts request ID from `X-Request-ID` header or generates a new UUID.
/// Stores the request ID in request extensions for use by other middleware/handlers.
///
/// # Example
///
/// ```rust
/// use actix_web::App;
/// use shared_lib::middleware::RequestIdMiddleware;
///
/// let app = App::new()
///     .wrap(RequestIdMiddleware)
///     // ... other middleware and routes
///     ;
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RequestIdMiddleware;

/// Request ID stored in request extensions
#[derive(Debug, Clone)]
pub struct RequestId(pub String);

impl RequestId {
    /// Generate a new request ID
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Parse from string (e.g., from header)
    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Get the request ID as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

// Middleware factory
impl<S, B> Transform<S, ServiceRequest> for RequestIdMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestIdMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestIdMiddlewareService { service }))
    }
}

pub struct RequestIdMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestIdMiddlewareService<S>
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
        // Extract or generate request ID
        let request_id = req
            .headers()
            .get("X-Request-ID")
            .and_then(|h| h.to_str().ok())
            .map(|s| RequestId::from_string(s.to_string()))
            .unwrap_or_else(RequestId::new);

        // Store in request extensions
        req.extensions_mut().insert(request_id.clone());

        // Store for logging
        let request_id_str = request_id.0.clone();

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            // Add X-Request-ID to response headers
            if let Ok(header_value) = HeaderValue::from_str(&request_id_str) {
                res.headers_mut().insert(
                    actix_web::http::header::HeaderName::from_static("x-request-id"),
                    header_value,
                );
            }

            Ok(res)
        })
    }
}

/// Helper to extract request ID from request extensions
pub fn get_request_id(req: &actix_web::HttpRequest) -> Option<String> {
    req.extensions().get::<RequestId>().map(|rid| rid.0.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App, HttpResponse};

    #[actix_web::test]
    async fn test_request_id_generated() {
        let app = test::init_service(App::new().wrap(RequestIdMiddleware).route(
            "/test",
            web::get().to(|| async { HttpResponse::Ok().finish() }),
        ))
        .await;

        let req = test::TestRequest::get().uri("/test").to_request();
        let resp = test::call_service(&app, req).await;

        // Response should have X-Request-ID header
        assert!(resp.headers().get("x-request-id").is_some());
    }

    #[actix_web::test]
    async fn test_request_id_from_header() {
        let app = test::init_service(App::new().wrap(RequestIdMiddleware).route(
            "/test",
            web::get().to(|| async { HttpResponse::Ok().finish() }),
        ))
        .await;

        let test_id = "test-request-id-123";
        let req = test::TestRequest::get()
            .uri("/test")
            .insert_header(("X-Request-ID", test_id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Response should echo the same request ID
        let response_id = resp
            .headers()
            .get("x-request-id")
            .and_then(|h| h.to_str().ok())
            .unwrap();
        assert_eq!(response_id, test_id);
    }
}
