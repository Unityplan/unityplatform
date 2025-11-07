use crate::{models::user::User, services::TokenService};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use sqlx::PgPool;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

/// Authenticated user information extracted from JWT
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: uuid::Uuid,
    pub username: String,
    pub territory_code: String,
    pub public_key_hash: String,
}

/// Middleware factory for JWT authentication
pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Extract Authorization header
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok());

            let token = match auth_header {
                Some(header) if header.starts_with("Bearer ") => &header[7..],
                _ => return Err(ErrorUnauthorized("Missing or invalid Authorization header")),
            };

            // Get TokenService from app data
            let token_service = req
                .app_data::<actix_web::web::Data<TokenService>>()
                .ok_or_else(|| ErrorUnauthorized("TokenService not configured"))?;

            // Validate and decode JWT
            let claims = token_service
                .validate_token(token)
                .map_err(|_| ErrorUnauthorized("Invalid or expired token"))?;

            // Parse user_id from string to UUID
            let user_id = uuid::Uuid::parse_str(&claims.user_id)
                .map_err(|_| ErrorUnauthorized("Invalid user ID in token"))?;

            // Get database pool
            let pool = req
                .app_data::<actix_web::web::Data<PgPool>>()
                .ok_or_else(|| ErrorUnauthorized("Database pool not configured"))?;

            // Load user from database (using territory schema)
            let schema_name = format!("territory_{}", claims.territory_code.to_lowercase());
            let user_query = format!(
                r#"
                SELECT 
                    id, public_key_hash, email, password_hash, username, 
                    full_name, display_name, avatar_url, bio,
                    email_visible, profile_public, data_export_requested,
                    is_verified, is_active, last_login_at,
                    invited_by_token_id,
                    created_at, updated_at
                FROM {}.users 
                WHERE id = $1 AND is_active = true
                "#,
                schema_name
            );

            let user = sqlx::query_as::<_, User>(&user_query)
                .bind(user_id)
                .fetch_optional(pool.get_ref())
                .await
                .map_err(|e| {
                    tracing::error!("Database error loading user: {:?}", e);
                    ErrorUnauthorized("Failed to load user")
                })?
                .ok_or_else(|| ErrorUnauthorized("User not found or inactive"))?;

            // Store authenticated user in request extensions
            req.extensions_mut().insert(AuthenticatedUser {
                user_id: user.id,
                username: user.username,
                territory_code: claims.territory_code,
                public_key_hash: user.public_key_hash.unwrap_or_default(),
            });

            // Continue with request
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}

/// Helper function to extract authenticated user from request
pub fn get_authenticated_user(req: &actix_web::HttpRequest) -> Result<AuthenticatedUser, Error> {
    req.extensions()
        .get::<AuthenticatedUser>()
        .cloned()
        .ok_or_else(|| ErrorUnauthorized("User not authenticated"))
}
