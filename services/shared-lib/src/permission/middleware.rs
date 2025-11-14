//! Permission enforcement middleware
//!
//! Actix-web middleware for checking user permissions before allowing access to endpoints.

use crate::permission::PermissionChecker;
use crate::AppError;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::rc::Rc;
use tracing::{debug, warn};
use uuid::Uuid;

/// Middleware factory for requiring a specific permission
///
/// # Example
/// ```no_run
/// use actix_web::{web, App, HttpResponse};
/// use shared_lib::permission::RequirePermission;
/// use shared_lib::PermissionChecker;
///
/// async fn admin_only() -> HttpResponse {
///     HttpResponse::Ok().body("Admin access granted")
/// }
///
/// fn configure(
///     cfg: &mut web::ServiceConfig,
///     permission_checker: PermissionChecker,
/// ) {
///     cfg.app_data(web::Data::new(permission_checker))
///         .service(
///             web::resource("/admin")
///                 .wrap(RequirePermission::new("portal:manage"))
///                 .route(web::get().to(admin_only))
///         );
/// }
/// ```
pub struct RequirePermission {
    permission: String,
}

impl RequirePermission {
    /// Create middleware that requires a specific permission
    pub fn new(permission: impl Into<String>) -> Self {
        Self {
            permission: permission.into(),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequirePermission
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequirePermissionMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequirePermissionMiddleware {
            service: Rc::new(service),
            permission: self.permission.clone(),
        }))
    }
}

pub struct RequirePermissionMiddleware<S> {
    service: Rc<S>,
    permission: String,
}

impl<S, B> Service<ServiceRequest> for RequirePermissionMiddleware<S>
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
        let permission = self.permission.clone();
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Extract user_id from request extensions (set by JWT middleware)
            let user_id = req.extensions().get::<Uuid>().copied();

            let user_id = match user_id {
                Some(id) => id,
                None => {
                    warn!("Permission check failed: No user_id in request");
                    return Err(
                        AppError::Unauthorized("Authentication required".to_string()).into(),
                    );
                }
            };

            // Extract PermissionChecker from app data
            let permission_checker = req
                .app_data::<actix_web::web::Data<PermissionChecker>>()
                .map(|d| d.get_ref().clone());

            let permission_checker = match permission_checker {
                Some(c) => c,
                None => {
                    warn!("PermissionChecker not found in app data");
                    return Err(
                        AppError::Internal("Permission system not configured".to_string()).into(),
                    );
                }
            };

            // Check permission
            let has_permission = permission_checker
                .has_permission(user_id, &permission)
                .await
                .map_err(|e| {
                    warn!("Permission check error: {}", e);
                    AppError::Internal("Permission check failed".to_string())
                })?;

            if !has_permission {
                warn!(
                    user_id = %user_id,
                    required_permission = %permission,
                    "User lacks required permission"
                );
                return Err(AppError::Forbidden(format!(
                    "Missing required permission: {}",
                    permission
                ))
                .into());
            }

            debug!(
                user_id = %user_id,
                permission = %permission,
                "Permission granted"
            );

            // Continue with request
            service.call(req).await
        })
    }
}

/// Middleware factory for requiring ANY of multiple permissions
///
/// # Example
/// ```no_run
/// use actix_web::{web, App, HttpResponse};
/// use shared_lib::permission::RequireAnyPermission;
/// use shared_lib::PermissionChecker;
///
/// async fn moderator_or_admin() -> HttpResponse {
///     HttpResponse::Ok().body("Moderator or admin access granted")
/// }
///
/// fn configure(
///     cfg: &mut web::ServiceConfig,
///     permission_checker: PermissionChecker,
/// ) {
///     cfg.app_data(web::Data::new(permission_checker))
///         .service(
///             web::resource("/moderate")
///                 .wrap(RequireAnyPermission::new(vec![
///                     "content:moderate",
///                     "portal:manage",
///                 ]))
///                 .route(web::get().to(moderator_or_admin))
///         );
/// }
/// ```
pub struct RequireAnyPermission {
    permissions: Vec<String>,
}

impl RequireAnyPermission {
    /// Create middleware that requires any of the specified permissions
    pub fn new(permissions: Vec<&str>) -> Self {
        Self {
            permissions: permissions.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequireAnyPermission
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequireAnyPermissionMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequireAnyPermissionMiddleware {
            service: Rc::new(service),
            permissions: self.permissions.clone(),
        }))
    }
}

pub struct RequireAnyPermissionMiddleware<S> {
    service: Rc<S>,
    permissions: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for RequireAnyPermissionMiddleware<S>
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
        let permissions = self.permissions.clone();
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Extract user_id from request extensions (set by JWT middleware)
            let user_id = req.extensions().get::<Uuid>().copied();

            let user_id = match user_id {
                Some(id) => id,
                None => {
                    warn!("Permission check failed: No user_id in request");
                    return Err(
                        AppError::Unauthorized("Authentication required".to_string()).into(),
                    );
                }
            };

            // Extract PermissionChecker from app data
            let permission_checker = req
                .app_data::<actix_web::web::Data<PermissionChecker>>()
                .map(|d| d.get_ref().clone());

            let permission_checker = match permission_checker {
                Some(c) => c,
                None => {
                    warn!("PermissionChecker not found in app data");
                    return Err(
                        AppError::Internal("Permission system not configured".to_string()).into(),
                    );
                }
            };

            // Check if user has any of the required permissions
            let perm_refs: Vec<&str> = permissions.iter().map(|s| s.as_str()).collect();
            let has_permission = permission_checker
                .has_any_permission(user_id, &perm_refs)
                .await
                .map_err(|e| {
                    warn!("Permission check error: {}", e);
                    AppError::Internal("Permission check failed".to_string())
                })?;

            if !has_permission {
                warn!(
                    user_id = %user_id,
                    required_permissions = ?permissions,
                    "User lacks all required permissions"
                );
                return Err(AppError::Forbidden(format!(
                    "Missing required permissions. Need one of: {}",
                    permissions.join(", ")
                ))
                .into());
            }

            debug!(
                user_id = %user_id,
                permissions = ?permissions,
                "Permission granted (has at least one)"
            );

            // Continue with request
            service.call(req).await
        })
    }
}
