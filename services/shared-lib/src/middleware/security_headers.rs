use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

/// Security headers middleware
///
/// Adds common security headers to all responses:
/// - X-Content-Type-Options: nosniff
/// - X-Frame-Options: DENY
/// - X-XSS-Protection: 1; mode=block
/// - Referrer-Policy: strict-origin-when-cross-origin
/// - Content-Security-Policy (Phase 2 only)
/// - Strict-Transport-Security (Phase 2 only)
///
/// # Phase 1 (Development)
/// - Basic security headers
/// - No CSP (for easier development)
/// - No HSTS (localhost doesn't use HTTPS)
///
/// # Phase 2 (Production)
/// - All security headers
/// - Strict CSP
/// - HSTS with long max-age
///
/// # Example
///
/// ```rust
/// use actix_web::App;
/// use shared_lib::middleware::SecurityHeadersMiddleware;
///
/// let app = App::new()
///     .wrap(SecurityHeadersMiddleware::production())
///     // ... routes
///     ;
/// ```
#[derive(Debug, Clone)]
pub struct SecurityHeadersMiddleware {
    /// Enable strict security headers (Phase 2)
    strict: bool,
    /// Custom CSP policy (optional)
    csp_policy: Option<String>,
}

impl SecurityHeadersMiddleware {
    /// Create new security headers middleware
    ///
    /// # Arguments
    /// * `strict` - true for production, false for development
    pub fn new(strict: bool) -> Self {
        Self {
            strict,
            csp_policy: None,
        }
    }

    /// Development - Basic security headers
    pub fn development() -> Self {
        Self {
            strict: false,
            csp_policy: None,
        }
    }

    /// Production - All security headers with strict CSP, HSTS, and Permissions-Policy
    pub fn production() -> Self {
        Self {
            strict: true,
            csp_policy: Some(
                "default-src 'self'; \
                 script-src 'self'; \
                 style-src 'self' 'unsafe-inline'; \
                 img-src 'self' data: https:; \
                 font-src 'self' data:; \
                 connect-src 'self'; \
                 frame-ancestors 'none'; \
                 base-uri 'self'; \
                 form-action 'self'"
                    .to_string(),
            ),
        }
    }

    /// Set custom CSP policy
    pub fn with_csp(mut self, policy: impl Into<String>) -> Self {
        self.csp_policy = Some(policy.into());
        self
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

impl Default for SecurityHeadersMiddleware {
    fn default() -> Self {
        // Default to production for safety
        Self::production()
    }
}

// Middleware factory
impl<S, B> Transform<S, ServiceRequest> for SecurityHeadersMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityHeadersMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SecurityHeadersMiddlewareService {
            service,
            strict: self.strict,
            csp_policy: self.csp_policy.clone(),
        }))
    }
}

pub struct SecurityHeadersMiddlewareService<S> {
    service: S,
    strict: bool,
    csp_policy: Option<String>,
}

impl<S, B> Service<ServiceRequest> for SecurityHeadersMiddlewareService<S>
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
        let strict = self.strict;
        let csp_policy = self.csp_policy.clone();

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            // Add security headers to response
            let headers = res.headers_mut();

            // Basic security headers (Phase 1 & Phase 2)
            headers.insert(
                actix_web::http::header::HeaderName::from_static("x-content-type-options"),
                actix_web::http::header::HeaderValue::from_static("nosniff"),
            );

            headers.insert(
                actix_web::http::header::HeaderName::from_static("x-frame-options"),
                actix_web::http::header::HeaderValue::from_static("DENY"),
            );

            headers.insert(
                actix_web::http::header::HeaderName::from_static("x-xss-protection"),
                actix_web::http::header::HeaderValue::from_static("1; mode=block"),
            );

            headers.insert(
                actix_web::http::header::HeaderName::from_static("referrer-policy"),
                actix_web::http::header::HeaderValue::from_static(
                    "strict-origin-when-cross-origin",
                ),
            );

            // Strict headers (Phase 2 only)
            if strict {
                // Content Security Policy
                if let Some(csp) = csp_policy {
                    if let Ok(value) = actix_web::http::header::HeaderValue::from_str(&csp) {
                        headers.insert(
                            actix_web::http::header::HeaderName::from_static(
                                "content-security-policy",
                            ),
                            value,
                        );
                    }
                }

                // HSTS (only in production with HTTPS)
                headers.insert(
                    actix_web::http::header::HeaderName::from_static("strict-transport-security"),
                    actix_web::http::header::HeaderValue::from_static(
                        "max-age=31536000; includeSubDomains",
                    ),
                );

                // Permissions Policy (formerly Feature-Policy)
                headers.insert(
                    actix_web::http::header::HeaderName::from_static("permissions-policy"),
                    actix_web::http::header::HeaderValue::from_static(
                        "geolocation=(), microphone=(), camera=()",
                    ),
                );
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
    async fn test_security_headers_phase1() {
        let app = test::init_service(
            App::new()
                .wrap(SecurityHeadersMiddleware::development())
                .route(
                    "/test",
                    web::get().to(|| async { HttpResponse::Ok().finish() }),
                ),
        )
        .await;

        let req = test::TestRequest::get().uri("/test").to_request();
        let resp = test::call_service(&app, req).await;

        // Should have basic security headers
        assert!(resp.headers().get("x-content-type-options").is_some());
        assert!(resp.headers().get("x-frame-options").is_some());
        assert!(resp.headers().get("x-xss-protection").is_some());
        assert!(resp.headers().get("referrer-policy").is_some());

        // Should NOT have strict headers
        assert!(resp.headers().get("content-security-policy").is_none());
        assert!(resp.headers().get("strict-transport-security").is_none());
    }

    #[actix_web::test]
    async fn test_security_headers_phase2() {
        let app = test::init_service(
            App::new()
                .wrap(SecurityHeadersMiddleware::production())
                .route(
                    "/test",
                    web::get().to(|| async { HttpResponse::Ok().finish() }),
                ),
        )
        .await;

        let req = test::TestRequest::get().uri("/test").to_request();
        let resp = test::call_service(&app, req).await;

        // Should have all security headers
        assert!(resp.headers().get("x-content-type-options").is_some());
        assert!(resp.headers().get("x-frame-options").is_some());
        assert!(resp.headers().get("x-xss-protection").is_some());
        assert!(resp.headers().get("referrer-policy").is_some());
        assert!(resp.headers().get("content-security-policy").is_some());
        assert!(resp.headers().get("strict-transport-security").is_some());
        assert!(resp.headers().get("permissions-policy").is_some());
    }

    #[actix_web::test]
    async fn test_custom_csp() {
        let custom_csp = "default-src 'self'; script-src 'self' 'unsafe-inline'";
        let app = test::init_service(
            App::new()
                .wrap(SecurityHeadersMiddleware::production().with_csp(custom_csp))
                .route(
                    "/test",
                    web::get().to(|| async { HttpResponse::Ok().finish() }),
                ),
        )
        .await;

        let req = test::TestRequest::get().uri("/test").to_request();
        let resp = test::call_service(&app, req).await;

        let csp = resp
            .headers()
            .get("content-security-policy")
            .and_then(|h| h.to_str().ok());
        assert_eq!(csp, Some(custom_csp));
    }
}
