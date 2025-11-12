use crate::{error::AuthError, services::TokenService};
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
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
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        Box::pin(async move {
            let token_service = req
                .app_data::<actix_web::web::Data<TokenService>>()
                .ok_or_else(|| {
                    actix_web::error::ErrorInternalServerError("TokenService not configured")
                })?;

            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .ok_or_else(|| {
                    actix_web::error::ErrorUnauthorized("Missing Authorization header")
                })?;

            let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
                actix_web::error::ErrorUnauthorized("Invalid Authorization header")
            })?;

            let claims = token_service.validate_token(token).map_err(|e| match e {
                AuthError::TokenExpired => actix_web::error::ErrorUnauthorized("Token has expired"),
                AuthError::InvalidToken(_) => actix_web::error::ErrorUnauthorized("Invalid token"),
                _ => actix_web::error::ErrorUnauthorized("Authentication failed"),
            })?;

            req.extensions_mut().insert(claims);

            let res = srv.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}
