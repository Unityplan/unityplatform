use crate::{
    middleware::get_authenticated_user,
    models::invitation::{CreateInvitationRequest, InvitationResponse},
    services::{
        create_invitation_token, get_invitation_uses, get_token_territory, list_user_invitations,
        revoke_invitation_token, validate_invitation_token,
    },
};
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

/// Get schema name for a territory
/// For single-territory pods: returns "territory"
/// For multi-territory pods: returns "territory_XX" (e.g., "territory_de")
fn get_schema_name(_territory_code: &str) -> String {
    // TODO: Make this configurable via environment variable
    // For now, use single-territory approach (generic "territory" schema)
    "territory".to_string()

    // For multi-territory pods, use:
    // format!("territory_{}", territory_code.to_lowercase())
}

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
    let schema_name = get_schema_name(&auth_user.territory_code);

    // Create invitation token (⭐ Now includes territory_code for global registry)
    let token = create_invitation_token(
        pool.get_ref(),
        &schema_name,
        &auth_user.territory_code,  // ⭐ NEW: Pass territory for global registry
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
    let schema_name = get_schema_name(&auth_user.territory_code);

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
    let schema_name = get_schema_name(&auth_user.territory_code);

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
    let schema_name = get_schema_name(&auth_user.territory_code);

    let token_id = path.into_inner();

    // Get usage statistics
    let uses = get_invitation_uses(pool.get_ref(), &schema_name, token_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(uses))
}

/// Validate an invitation token (public endpoint - no auth required)
/// GET /api/auth/invitations/validate/{token}
/// 
/// ⭐ SECURE: Territory is looked up from global.invitation_token_registry
/// Client cannot manipulate which territory the token belongs to
pub async fn validate_invitation(
    path: web::Path<String>,
    query: web::Query<ValidationQuery>,
    pool: web::Data<PgPool>,
) -> actix_web::Result<HttpResponse> {
    let token = path.into_inner();

    // ⭐ SECURITY: Look up territory from global registry (client cannot manipulate this)
    let territory_code = get_token_territory(pool.get_ref(), &token)
        .await
        .map_err(|e| match e {
            shared_lib::error::AppError::Validation(msg) => {
                actix_web::error::ErrorBadRequest(msg)
            }
            _ => actix_web::error::ErrorInternalServerError(e),
        })?;

    let schema_name = get_schema_name(&territory_code);

    // Validate token details (expiration, uses, email matching)
    let invitation =
        validate_invitation_token(pool.get_ref(), &schema_name, &token, query.email.as_deref())
            .await
            .map_err(|e| match e {
                shared_lib::error::AppError::Validation(msg) => {
                    actix_web::error::ErrorBadRequest(msg)
                }
                _ => actix_web::error::ErrorInternalServerError(e),
            })?;

    // Lookup territory name from global.territories
    let territory_name = sqlx::query_scalar::<_, String>(
        "SELECT name FROM global.territories WHERE code = $1"
    )
    .bind(&territory_code)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .unwrap_or_else(|| territory_code.clone());

    // Lookup community name if invitation has community_id
    let community_info = if let Some(community_id) = invitation.community_id {
        let community_query = format!(
            "SELECT name FROM {}.communities WHERE id = $1",
            schema_name
        );
        
        let community_name = sqlx::query_scalar::<_, String>(&community_query)
            .bind(community_id)
            .fetch_optional(pool.get_ref())
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        community_name.map(|name| serde_json::json!({
            "id": community_id,
            "name": name
        }))
    } else {
        None
    };

    // Return validation response with territory and community info
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "valid": true,
        "token_type": invitation.token_type,
        "territory": {
            "code": territory_code,
            "name": territory_name
        },
        "community": community_info,
        "email": invitation.invited_email,
        "expires_at": invitation.expires_at,
        "remaining_uses": invitation.max_uses.map(|max| max - invitation.current_uses),
    })))
}

#[derive(serde::Deserialize)]
pub struct ValidationQuery {
    // ⭐ territory_code is NO LONGER needed - we look it up from global registry
    pub email: Option<String>,
}
