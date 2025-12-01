use actix_cors::Cors;
use actix_web::http::header;

/// CORS (Cross-Origin Resource Sharing) middleware configuration
///
/// Provides different CORS policies for development and production:
///
/// # Phase 1 (Development)
/// - Allow all origins (*)
/// - Allow all methods
/// - Allow all headers
/// - Allow credentials
/// - No max age restriction
///
/// # Phase 2 (Production)
/// - Whitelist specific origins only
/// - Specific methods (GET, POST, PUT, PATCH, DELETE)
/// - Specific headers
/// - Allow credentials
/// - Max age: 3600 seconds (1 hour)
///
/// # Example
///
/// ```rust,no_run
/// use actix_web::{web, App, HttpServer};
/// use shared_lib::middleware::cors;
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     // Development mode
///     HttpServer::new(|| {
///         App::new()
///             .wrap(cors::development())
///             // ... routes
///     })
///     .bind(("127.0.0.1", 8080))?
///     .run()
///     .await
/// }
/// ```
///
/// ```rust,no_run
/// use actix_web::{App, HttpServer};
/// use shared_lib::middleware::cors;
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     // Production mode
///     let allowed_origins = vec![
///         "https://unityplan.org".to_string(),
///         "https://app.unityplan.org".to_string(),
///     ];
///
///     HttpServer::new(move || {
///         App::new()
///             .wrap(cors::production(allowed_origins.clone()))
///             // ... routes
///     })
///     .bind(("0.0.0.0", 8080))?
///     .run()
///     .await
/// }
/// ```

/// Phase 1 (Development) - Permissive CORS for localhost development
///
/// Allows:
/// - All origins
/// - All methods
/// - All headers
/// - Credentials
pub fn development() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .supports_credentials()
        .max_age(3600)
}

/// Production - Strict CORS with origin whitelist
///
/// # Arguments
/// * `allowed_origins` - List of allowed origin URLs (e.g., ["https://app.example.com"])
///
/// Allows:
/// - Only specified origins
/// - Common HTTP methods (GET, POST, PUT, PATCH, DELETE, OPTIONS)
/// - Common headers (Authorization, Content-Type, Accept, X-Request-ID)
/// - Credentials
/// - Max age: 3600 seconds
pub fn production(allowed_origins: Vec<String>) -> Cors {
    let mut cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::HeaderName::from_static("x-request-id"),
        ])
        .supports_credentials()
        .max_age(3600);

    // Add each allowed origin
    for origin in allowed_origins {
        cors = cors.allowed_origin(&origin);
    }

    cors
}

/// Alias for development() - kept for backward compatibility
#[deprecated(since = "0.1.0-alpha.1", note = "Use development() instead")]
pub fn phase1() -> Cors {
    development()
}

/// Alias for production() - kept for backward compatibility
#[deprecated(since = "0.1.0-alpha.1", note = "Use production() instead")]
pub fn phase2(allowed_origins: Vec<String>) -> Cors {
    production(allowed_origins)
}

/// Custom CORS configuration
///
/// Provides a builder for creating custom CORS configurations
///
/// # Example
///
/// ```rust
/// use shared_lib::middleware::cors::CorsConfig;
///
/// let cors = CorsConfig::new()
///     .allow_origin("https://app.example.com")
///     .allow_origin("https://admin.example.com")
///     .allow_methods(vec!["GET", "POST"])
///     .allow_credentials(true)
///     .build();
/// ```
pub struct CorsConfig {
    origins: Vec<String>,
    methods: Vec<String>,
    headers: Vec<header::HeaderName>,
    allow_credentials: bool,
    max_age: usize,
}

impl CorsConfig {
    /// Create new CORS configuration builder
    pub fn new() -> Self {
        Self {
            origins: vec![],
            methods: vec!["GET".to_string(), "POST".to_string()],
            headers: vec![header::CONTENT_TYPE, header::AUTHORIZATION],
            allow_credentials: false,
            max_age: 3600,
        }
    }

    /// Add allowed origin
    pub fn allow_origin(mut self, origin: impl Into<String>) -> Self {
        self.origins.push(origin.into());
        self
    }

    /// Set allowed methods
    pub fn allow_methods(mut self, methods: Vec<&str>) -> Self {
        self.methods = methods.into_iter().map(|m| m.to_string()).collect();
        self
    }

    /// Add allowed header
    pub fn allow_header(mut self, header: header::HeaderName) -> Self {
        self.headers.push(header);
        self
    }

    /// Set whether to allow credentials
    pub fn allow_credentials(mut self, allow: bool) -> Self {
        self.allow_credentials = allow;
        self
    }

    /// Set max age in seconds
    pub fn max_age(mut self, seconds: usize) -> Self {
        self.max_age = seconds;
        self
    }

    /// Build the CORS middleware
    pub fn build(self) -> Cors {
        let mut cors = Cors::default().max_age(self.max_age);

        // Add origins
        if self.origins.is_empty() {
            cors = cors.allow_any_origin();
        } else {
            for origin in self.origins {
                cors = cors.allowed_origin(&origin);
            }
        }

        // Add methods
        let methods: Vec<&str> = self.methods.iter().map(|s| s.as_str()).collect();
        cors = cors.allowed_methods(methods);

        // Add headers
        cors = cors.allowed_headers(self.headers);

        // Credentials
        if self.allow_credentials {
            cors = cors.supports_credentials();
        }

        cors
    }
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cors_config_builder() {
        let config = CorsConfig::new()
            .allow_origin("https://example.com")
            .allow_methods(vec!["GET", "POST", "PUT"])
            .allow_credentials(true)
            .max_age(7200);

        assert_eq!(config.origins.len(), 1);
        assert_eq!(config.methods.len(), 3);
        assert!(config.allow_credentials);
        assert_eq!(config.max_age, 7200);
    }

    #[test]
    fn test_development_cors() {
        let _cors = development();
        // Just ensure it builds without panicking
    }

    #[test]
    fn test_production_cors() {
        let origins = vec!["https://app.example.com".to_string()];
        let _cors = production(origins);
        // Just ensure it builds without panicking
    }
}
