use actix_web::{web, HttpResponse};
use shared_lib::{AppConfig, AuthUser, Database, NatsClient, ValidatedJson};
use tracing::info;

use crate::models::invitation::{
    CreateInvitationRequest, CreateInvitationResponse, GetInvitationUsesResponse,
    ListInvitationsQuery, ListInvitationsResponse, RevokeInvitationResponse,
};
use crate::models::usage::{UseInvitationRequest, UseInvitationResponse};
use crate::models::validation::{ValidateInvitationRequest, ValidateInvitationResponse};
use crate::services::invitation::{
    create_invitation, get_invitation_uses, list_user_invitations, revoke_invitation,
    use_invitation, validate_invitation,
};
use crate::services::permissions::check_manager_permissions;

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

/// Use invitation token after successful registration
///
/// Called by auth-service AFTER successful user registration to record usage.
/// This endpoint requires authentication from auth-service (service-to-service).
#[utoipa::path(
    post,
    path = "/api/v1/invitations/use",
    tag = "invitations",
    request_body = UseInvitationRequest,
    responses(
        (status = 200, description = "Usage recorded successfully", body = UseInvitationResponse),
        (status = 400, description = "Invalid request or token fully used"),
        (status = 404, description = "Invalid or expired token"),
        (status = 409, description = "User already used this token"),
    )
)]
pub async fn use_invitation_handler(
    database: web::Data<Database>,
    config: web::Data<AppConfig>,
    nats: web::Data<NatsClient>,
    body: ValidatedJson<UseInvitationRequest>,
) -> HttpResponse {
    let token = &body.token;
    let used_by = body.used_by;
    let territory_code = &config.server.territory;

    info!(
        token = %token,
        used_by = %used_by,
        territory = %territory_code,
        "Recording invitation usage"
    );

    // Record usage
    match use_invitation(
        database.pool(),
        token,
        used_by,
        territory_code,
        body.ip_address.clone(),
        body.user_agent.clone(),
    )
    .await
    {
        Ok(response) => {
            info!(
                invitation_id = %response.invitation_id,
                uses_remaining = response.uses_remaining,
                fully_used = response.fully_used,
                "Invitation usage recorded successfully"
            );

            // Publish NATS event: invitation.used
            let event_payload = serde_json::json!({
                "invitation_id": response.invitation_id,
                "used_by": used_by,
                "token": token,
                "territory": territory_code,
                "uses_remaining": response.uses_remaining,
                "fully_used": response.fully_used,
                "timestamp": chrono::Utc::now(),
            });

            if let Err(e) = nats
                .publish(
                    "invitation.used",
                    event_payload.to_string().as_bytes().to_vec(),
                )
                .await
            {
                tracing::warn!(
                    error = %e,
                    "Failed to publish invitation.used event (non-critical)"
                );
            }

            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to record invitation usage");
            HttpResponse::from(actix_web::Error::from(e))
        }
    }
}

/// Create invitation token
///
/// Creates a new invitation token. Requires manager permissions.
/// Only territory managers or community managers assigned to this territory can create invitations.
#[utoipa::path(
    post,
    path = "/api/v1/invitations",
    tag = "invitations",
    request_body = CreateInvitationRequest,
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 201, description = "Invitation created successfully", body = CreateInvitationResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized - missing or invalid JWT"),
        (status = 403, description = "Forbidden - user is not a manager"),
    )
)]
pub async fn create_invitation_handler(
    database: web::Data<Database>,
    config: web::Data<AppConfig>,
    nats: web::Data<NatsClient>,
    auth_user: AuthUser,
    body: ValidatedJson<CreateInvitationRequest>,
) -> HttpResponse {
    let territory_code = &config.server.territory;

    info!(
        user_id = %auth_user.id,
        user_territory = %auth_user.territory,
        target_territory = %territory_code,
        "Manager attempting to create invitation"
    );

    // Check if user has manager permissions for this territory
    match check_manager_permissions(database.pool(), auth_user.id, territory_code).await {
        Ok(true) => {
            info!(
                user_id = %auth_user.id,
                territory = %territory_code,
                "Manager permissions verified"
            );
        }
        Ok(false) => {
            info!(
                user_id = %auth_user.id,
                territory = %territory_code,
                "User does not have manager permissions"
            );
            return HttpResponse::Forbidden().json(serde_json::json!({
                "error": "Forbidden",
                "message": "You do not have manager permissions for this territory"
            }));
        }
        Err(e) => {
            tracing::error!(
                error = %e,
                user_id = %auth_user.id,
                territory = %territory_code,
                "Failed to check manager permissions"
            );
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to verify permissions"
            }));
        }
    }

    // Create invitation
    match create_invitation(
        database.pool(),
        auth_user.id,
        territory_code,
        body.max_uses,
        body.expires_in_days,
        body.metadata.clone(),
    )
    .await
    {
        Ok(invitation) => {
            info!(
                invitation_id = %invitation.id,
                token = %invitation.token,
                created_by = %invitation.created_by,
                max_uses = invitation.max_uses,
                "Invitation created successfully"
            );

            // Build invite URL
            let base_url = std::env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:5173".to_string());
            let invite_url = format!("{}/register?token={}", base_url, invitation.token);

            // Publish NATS event: invitation.created
            let event_payload = serde_json::json!({
                "invitation_id": invitation.id,
                "token": invitation.token,
                "created_by": invitation.created_by,
                "territory": territory_code,
                "max_uses": invitation.max_uses,
                "expires_at": invitation.expires_at,
                "timestamp": chrono::Utc::now(),
            });

            if let Err(e) = nats
                .publish(
                    "invitation.created",
                    event_payload.to_string().as_bytes().to_vec(),
                )
                .await
            {
                tracing::warn!(
                    error = %e,
                    "Failed to publish invitation.created event (non-critical)"
                );
            }

            HttpResponse::Created().json(CreateInvitationResponse {
                id: invitation.id,
                token: invitation.token,
                created_by: invitation.created_by,
                max_uses: invitation.max_uses,
                expires_at: invitation.expires_at,
                invite_url,
            })
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to create invitation");
            HttpResponse::from(actix_web::Error::from(e))
        }
    }
}

/// List my invitations
///
/// Get all invitations created by the authenticated user with filtering and pagination.
#[utoipa::path(
    get,
    path = "/api/v1/invitations/me",
    tag = "invitations",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("status" = Option<String>, Query, description = "Filter by status (active, used, expired, revoked)"),
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("limit" = Option<i64>, Query, description = "Results per page (default: 20, max: 100)"),
    ),
    responses(
        (status = 200, description = "Invitations list", body = ListInvitationsResponse),
        (status = 401, description = "Unauthorized - missing or invalid JWT"),
    )
)]
pub async fn list_my_invitations_handler(
    database: web::Data<Database>,
    config: web::Data<AppConfig>,
    auth_user: AuthUser,
    query: web::Query<ListInvitationsQuery>,
) -> HttpResponse {
    let territory_code = &config.server.territory;

    info!(
        user_id = %auth_user.id,
        territory = %territory_code,
        status_filter = ?query.status,
        page = query.page,
        limit = query.limit,
        "Listing user's invitations"
    );

    match list_user_invitations(
        database.pool(),
        auth_user.id,
        territory_code,
        query.status.clone(),
        query.page,
        query.limit,
    )
    .await
    {
        Ok(response) => {
            info!(
                user_id = %auth_user.id,
                total = response.pagination.total,
                "Retrieved invitations list"
            );
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to list invitations");
            HttpResponse::from(actix_web::Error::from(e))
        }
    }
}

/// Get invitation uses
///
/// Get detailed usage list for a specific invitation.
/// Only the creator or a manager can view.
#[utoipa::path(
    get,
    path = "/api/v1/invitations/{id}/uses",
    tag = "invitations",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = uuid::Uuid, Path, description = "Invitation ID"),
    ),
    responses(
        (status = 200, description = "Invitation uses", body = GetInvitationUsesResponse),
        (status = 401, description = "Unauthorized - missing or invalid JWT"),
        (status = 403, description = "Forbidden - not creator or manager"),
        (status = 404, description = "Invitation not found"),
    )
)]
pub async fn get_invitation_uses_handler(
    database: web::Data<Database>,
    config: web::Data<AppConfig>,
    auth_user: AuthUser,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let invitation_id = path.into_inner();
    let territory_code = &config.server.territory;

    info!(
        user_id = %auth_user.id,
        invitation_id = %invitation_id,
        territory = %territory_code,
        "Retrieving invitation uses"
    );

    match get_invitation_uses(database.pool(), invitation_id, territory_code, auth_user.id).await {
        Ok(response) => {
            info!(
                invitation_id = %invitation_id,
                total_uses = response.total_uses,
                "Retrieved invitation uses"
            );
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to get invitation uses");
            HttpResponse::from(actix_web::Error::from(e))
        }
    }
}

/// Revoke invitation
///
/// Revoke an invitation token. Only the creator or a manager can revoke.
#[utoipa::path(
    delete,
    path = "/api/v1/invitations/{id}",
    tag = "invitations",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = uuid::Uuid, Path, description = "Invitation ID"),
    ),
    responses(
        (status = 200, description = "Invitation revoked successfully", body = RevokeInvitationResponse),
        (status = 401, description = "Unauthorized - missing or invalid JWT"),
        (status = 403, description = "Forbidden - not creator or manager"),
        (status = 404, description = "Invitation not found"),
        (status = 409, description = "Conflict - already revoked"),
    )
)]
pub async fn revoke_invitation_handler(
    database: web::Data<Database>,
    config: web::Data<AppConfig>,
    nats: web::Data<NatsClient>,
    auth_user: AuthUser,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let invitation_id = path.into_inner();
    let territory_code = &config.server.territory;

    info!(
        user_id = %auth_user.id,
        invitation_id = %invitation_id,
        territory = %territory_code,
        "Revoking invitation"
    );

    match revoke_invitation(database.pool(), invitation_id, territory_code, auth_user.id).await {
        Ok(response) => {
            info!(
                invitation_id = %invitation_id,
                revoked_at = %response.revoked_at,
                "Invitation revoked successfully"
            );

            // Publish NATS event: invitation.revoked
            let event_payload = serde_json::json!({
                "invitation_id": invitation_id,
                "revoked_by": auth_user.id,
                "territory": territory_code,
                "revoked_at": response.revoked_at,
                "timestamp": chrono::Utc::now(),
            });

            if let Err(e) = nats
                .publish(
                    "invitation.revoked",
                    event_payload.to_string().as_bytes().to_vec(),
                )
                .await
            {
                tracing::warn!(
                    error = %e,
                    "Failed to publish invitation.revoked event (non-critical)"
                );
            }

            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to revoke invitation");
            HttpResponse::from(actix_web::Error::from(e))
        }
    }
}
