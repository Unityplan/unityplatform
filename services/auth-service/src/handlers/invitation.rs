use crate::{
    middleware::get_authenticated_user,
    models::invitation::{CreateInvitationRequest, InvitationResponse},
    services::{
        create_invitation_token, get_invitation_uses, list_user_invitations,
        revoke_invitation_token, validate_invitation_token,
    },
};
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

/// Create a new invitation token
/// POST /api/auth/invitations
pub async fn create_invitation(
    req: HttpRequest,
    body: web::Json<CreateInvitationRequest>,
    pool: web::Data<PgPool>,
) -> actix_web::Result<HttpResponse> {
    // Get authenticated user
    let auth_user = get_authenticated_user(&req)?;

    // Validate request
    body.validate()
        .map_err(|e| actix_web::error::ErrorBadRequest(format!("Validation error: {}", e)))?;

    // Validate business rules
    body.validate_business_rules()
        .map_err(|e| actix_web::error::ErrorBadRequest(e))?;

    // Get territory schema
    let schema_name = format!("territory_{}", auth_user.territory_code.to_lowercase());

    // Create invitation token
    let token = create_invitation_token(
        pool.get_ref(),
        &schema_name,
        &body.token_type,
        body.email.clone(),
        body.max_uses,
        body.expires_in_days,
        body.purpose.clone(),
        Some(auth_user.user_id),
    )
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    // Return response
    Ok(HttpResponse::Created().json(InvitationResponse::from(token)))
}

/// List invitation tokens created by the authenticated user
/// GET /api/auth/invitations
pub async fn list_invitations(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> actix_web::Result<HttpResponse> {
    // Get authenticated user
    let auth_user = get_authenticated_user(&req)?;

    // Get territory schema
    let schema_name = format!("territory_{}", auth_user.territory_code.to_lowercase());

    // List user's invitations
    let tokens = list_user_invitations(pool.get_ref(), &schema_name, auth_user.user_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Convert to response format
    let responses: Vec<InvitationResponse> =
        tokens.into_iter().map(InvitationResponse::from).collect();

    Ok(HttpResponse::Ok().json(responses))
}

/// Revoke an invitation token
/// DELETE /api/auth/invitations/{id}
pub async fn revoke_invitation(
    req: HttpRequest,
    path: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> actix_web::Result<HttpResponse> {
    // Get authenticated user
    let auth_user = get_authenticated_user(&req)?;

    // Get territory schema
    let schema_name = format!("territory_{}", auth_user.territory_code.to_lowercase());

    let token_id = path.into_inner();

    // Revoke token (only if created by this user)
    revoke_invitation_token(pool.get_ref(), &schema_name, token_id, auth_user.user_id)
        .await
        .map_err(|e| match e {
            shared_lib::error::AppError::NotFound(msg) => actix_web::error::ErrorNotFound(msg),
            _ => actix_web::error::ErrorInternalServerError(e),
        })?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Invitation token revoked successfully"
    })))
}

/// Get usage statistics for an invitation token
/// GET /api/auth/invitations/{id}/uses
pub async fn get_invitation_usage(
    req: HttpRequest,
    path: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> actix_web::Result<HttpResponse> {
    // Get authenticated user
    let auth_user = get_authenticated_user(&req)?;

    // Get territory schema
    let schema_name = format!("territory_{}", auth_user.territory_code.to_lowercase());

    let token_id = path.into_inner();

    // Get usage statistics
    let uses = get_invitation_uses(pool.get_ref(), &schema_name, token_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(uses))
}

/// Validate an invitation token (public endpoint - no auth required)
/// GET /api/auth/invitations/validate/{token}
pub async fn validate_invitation(
    path: web::Path<String>,
    query: web::Query<ValidationQuery>,
    pool: web::Data<PgPool>,
) -> actix_web::Result<HttpResponse> {
    let token = path.into_inner();

    // For validation, we need to know which territory to check
    // This should come from query parameter
    let territory_code = query.territory_code.as_deref().ok_or_else(|| {
        actix_web::error::ErrorBadRequest("territory_code query parameter is required")
    })?;

    let schema_name = format!("territory_{}", territory_code.to_lowercase());

    // Validate token (without consuming it)
    let invitation =
        validate_invitation_token(pool.get_ref(), &schema_name, &token, query.email.as_deref())
            .await
            .map_err(|e| match e {
                shared_lib::error::AppError::Validation(msg) => {
                    actix_web::error::ErrorBadRequest(msg)
                }
                _ => actix_web::error::ErrorInternalServerError(e),
            })?;

    // Return validation response
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "valid": true,
        "token_type": invitation.token_type,
        "email": invitation.invited_email,
        "expires_at": invitation.expires_at,
        "remaining_uses": invitation.max_uses.map(|max| max - invitation.current_uses),
    })))
}

#[derive(serde::Deserialize)]
pub struct ValidationQuery {
    pub territory_code: Option<String>,
    pub email: Option<String>,
}
