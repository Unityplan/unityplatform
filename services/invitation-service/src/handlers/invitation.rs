use actix_web::{web, HttpResponse};
use shared_lib::{AppConfig, Database, ValidatedJson};
use tracing::info;

use crate::models::validation::{ValidateInvitationRequest, ValidateInvitationResponse};
use crate::services::invitation::validate_invitation;

/// Validate invitation token
///
/// Called by auth-service during registration to check if invitation token is valid.
/// This endpoint is public (no authentication required).
#[utoipa::path(
    post,
    path = "/api/v1/invitations/validate",
    tag = "invitations",
    request_body = ValidateInvitationRequest,
    responses(
        (status = 200, description = "Validation result", body = ValidateInvitationResponse),
        (status = 400, description = "Invalid request"),
    )
)]
pub async fn validate_invitation_handler(
    database: web::Data<Database>,
    config: web::Data<AppConfig>,
    body: ValidatedJson<ValidateInvitationRequest>,
) -> HttpResponse {
    let token = &body.token;
    
    // Get territory code from config
    let territory_code = &config.server.territory;
    
    info!(
        token = %token,
        territory = %territory_code,
        "Validating invitation token"
    );
    
    // Validate invitation
    match validate_invitation(database.pool(), token, territory_code).await {
        Ok(Some(invitation)) => {
            // Calculate uses remaining
            let uses_remaining = if invitation.max_uses == 0 {
                0 // Unlimited uses
            } else {
                invitation.max_uses - invitation.uses_count
            };
            
            info!(
                invitation_id = %invitation.id,
                created_by = %invitation.created_by,
                uses_remaining = uses_remaining,
                "Invitation token is valid"
            );
            
            HttpResponse::Ok().json(ValidateInvitationResponse::valid(
                invitation.id,
                invitation.created_by,
                uses_remaining,
                invitation.expires_at,
                invitation.metadata,
            ))
        }
        Ok(None) => {
            info!(token = %token, "Invitation token is invalid");
            HttpResponse::Ok().json(ValidateInvitationResponse::invalid())
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to validate invitation");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to validate invitation"
            }))
        }
    }
}
