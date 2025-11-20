use chrono::{Duration, Utc};
use rand::Rng;
use shared_lib::{AppError, Result};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::models::invitation::{
    GetInvitationUsesResponse, Invitation, InvitationUse, InvitationWithUses,
    ListInvitationsResponse, PaginationInfo, RevokeInvitationResponse,
};
use crate::models::usage::UseInvitationResponse;
use crate::services::permissions::check_manager_permissions;

/// Characters allowed in invitation tokens (excludes confusing: 0, O, I, 1, l)
const TOKEN_CHARS: &[u8] = b"23456789ABCDEFGHJKMNPQRSTUVWXYZ";

/// Generate a random invitation token in format XXXX-XXXX-XXXX-XXXX
///
/// - 16 alphanumeric characters (excluding confusing characters: 0, O, I, 1, l)
/// - Formatted with dashes for readability
///
/// Example: `A7K9-M2X4-P5W8-Q1Z3`
pub fn generate_token() -> String {
    let mut rng = rand::thread_rng();
    let mut token = String::with_capacity(19); // 16 chars + 3 dashes

    for i in 0..16 {
        if i > 0 && i % 4 == 0 {
            token.push('-');
        }
        let idx = rng.gen_range(0..TOKEN_CHARS.len());
        token.push(TOKEN_CHARS[idx] as char);
    }

    token
}

/// Check if a token exists in the global registry
async fn token_exists_globally(pool: &PgPool, token: &str) -> Result<bool> {
    let result = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM global.registry_invitation WHERE token = $1)",
    )
    .bind(token)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

/// Create a new invitation token with global uniqueness enforcement
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `created_by` - User ID creating the invitation (from JWT, references global.registry_username)
/// * `territory_code` - Territory code (e.g., "dk")
/// * `max_uses` - Maximum uses (1 = single-use, 0 = unlimited)
/// * `expires_in_days` - Expiration in days (None = never expires)
/// * `metadata` - Optional metadata JSON
///
/// # Returns
/// Created invitation with unique token
///
/// # Database Operations
/// 1. Generate unique token (retry if collision)
/// 2. Begin transaction
/// 3. Insert into `territory_{code}.invitation_invitations_tokens`
/// 4. Insert into `global.registry_invitation`
/// 5. Commit transaction (rollback on any error)
pub async fn create_invitation(
    pool: &PgPool,
    created_by: Uuid,
    territory_code: &str,
    max_uses: i32,
    expires_in_days: Option<i32>,
    metadata: Option<serde_json::Value>,
) -> Result<Invitation> {
    // Generate unique token (retry up to 5 times if collision)
    let mut token = generate_token();
    let mut attempts = 0;

    while token_exists_globally(pool, &token).await? {
        attempts += 1;
        if attempts >= 5 {
            return Err(AppError::Internal(
                "Failed to generate unique token after 5 attempts".to_string(),
            ));
        }
        token = generate_token();
    }

    // Calculate expiration timestamp
    let expires_at = expires_in_days.map(|days| Utc::now() + Duration::days(days as i64));

    // Begin transaction for atomic insert
    let mut tx = pool.begin().await?;

    // Insert into territory table
    let invitation = insert_invitation_territory(
        &mut tx,
        territory_code,
        &token,
        created_by,
        max_uses,
        expires_at,
        metadata,
    )
    .await?;

    // Insert into global registry
    insert_invitation_global(&mut tx, &token, territory_code, invitation.id).await?;

    // Commit transaction
    tx.commit().await?;

    Ok(invitation)
}

/// Insert invitation into territory-specific table
async fn insert_invitation_territory(
    tx: &mut Transaction<'_, Postgres>,
    territory_code: &str,
    token: &str,
    created_by: Uuid,
    max_uses: i32,
    expires_at: Option<chrono::DateTime<Utc>>,
    metadata: Option<serde_json::Value>,
) -> Result<Invitation> {
    let table_name = format!("territory_{}.invitation_invitations_tokens", territory_code);

    let query = format!(
        r#"
        INSERT INTO {} (token, created_by, max_uses, expires_at, metadata)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, token, created_by, max_uses, uses_count, expires_at, 
                  is_active, revoked_at, revoked_by, metadata, created_at, updated_at
        "#,
        table_name
    );

    let invitation = sqlx::query_as::<_, Invitation>(&query)
        .bind(token)
        .bind(created_by)
        .bind(max_uses)
        .bind(expires_at)
        .bind(metadata)
        .fetch_one(&mut **tx)
        .await?;

    Ok(invitation)
}

/// Insert invitation into global registry for uniqueness enforcement
async fn insert_invitation_global(
    tx: &mut Transaction<'_, Postgres>,
    token: &str,
    territory_code: &str,
    territory_token_id: Uuid,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO global.registry_invitation (token, territory_code, territory_token_id)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(token)
    .bind(territory_code)
    .bind(territory_token_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

/// Validate an invitation token for use during registration
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `token` - Invitation token to validate
/// * `territory_code` - Territory code (e.g., "dk")
///
/// # Returns
/// * `Ok(Some(Invitation))` - Token is valid and usable
/// * `Ok(None)` - Token is invalid, expired, or fully used
///
/// # Validation Rules
/// 1. Token exists in database
/// 2. `is_active = true`
/// 3. Not expired (`expires_at > NOW()` or NULL)
/// 4. Under usage limit (`uses_count < max_uses` or `max_uses = 0`)
pub async fn validate_invitation(
    pool: &PgPool,
    token: &str,
    territory_code: &str,
) -> Result<Option<Invitation>> {
    let table_name = format!("territory_{}.invitation_invitations_tokens", territory_code);

    let query = format!(
        r#"
        SELECT id, token, created_by, max_uses, uses_count, expires_at,
               is_active, revoked_at, revoked_by, metadata, created_at, updated_at
        FROM {}
        WHERE token = $1
          AND is_active = true
          AND (expires_at IS NULL OR expires_at > NOW())
          AND (max_uses = 0 OR uses_count < max_uses)
        "#,
        table_name
    );

    let invitation = sqlx::query_as::<_, Invitation>(&query)
        .bind(token)
        .fetch_optional(pool)
        .await?;

    // Security Check: Verify that the creator still exists AND has permissions
    // This prevents usage of "zombie" invitations from deleted or revoked managers
    if let Some(ref inv) = invitation {
        let has_permissions =
            check_manager_permissions(pool, inv.created_by, territory_code).await?;

        if !has_permissions {
            tracing::warn!(
                invitation_id = %inv.id,
                created_by = %inv.created_by,
                territory = %territory_code,
                "Security Alert: Attempt to use invitation from manager with revoked permissions"
            );
            // Return None to treat as invalid/not found
            return Ok(None);
        }
    }

    Ok(invitation)
}

/// Record invitation usage after successful user registration
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `token` - Invitation token that was used
/// * `used_by` - User ID who used the invitation (from auth-service)
/// * `territory_code` - Territory code (e.g., "dk")
/// * `ip_address` - Optional IP address of the user
/// * `user_agent` - Optional user agent string
///
/// # Returns
/// * `Ok(UseInvitationResponse)` - Usage recorded successfully
/// * `Err(AppError::NotFound)` - Invalid or expired token
/// * `Err(AppError::Conflict)` - User already used this token
/// * `Err(AppError::BadRequest)` - Token is fully used
///
/// # Database Operations (Transaction)
/// 1. Validate token exists and is active
/// 2. Check for duplicate usage (same user_id + invitation_id)
/// 3. Insert into `invitation_invitations_uses`
/// 4. Increment `uses_count` in `invitation_invitations_tokens`
/// 5. Set `is_active = false` if fully used (uses_count >= max_uses)
pub async fn use_invitation(
    pool: &PgPool,
    token: &str,
    used_by: Uuid,
    territory_code: &str,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<UseInvitationResponse> {
    let mut tx = pool.begin().await?;

    // Step 1: Validate token and get invitation details
    let invitation = validate_invitation_for_use(&mut tx, token, territory_code).await?;

    // Step 2: Check for duplicate usage
    check_duplicate_usage(&mut tx, invitation.id, used_by, territory_code).await?;

    // Step 3: Record usage in invitation_invitations_uses
    insert_invitation_use(
        &mut tx,
        invitation.id,
        used_by,
        territory_code,
        ip_address,
        user_agent,
    )
    .await?;

    // Step 4: Increment uses_count and update is_active if fully used
    let (uses_remaining, fully_used) =
        update_invitation_usage(&mut tx, invitation.id, invitation.max_uses, territory_code)
            .await?;

    // Commit transaction
    tx.commit().await?;

    Ok(UseInvitationResponse::new(
        invitation.id,
        uses_remaining,
        fully_used,
    ))
}

/// Validate invitation for use (similar to validate_invitation but within transaction)
async fn validate_invitation_for_use(
    tx: &mut Transaction<'_, Postgres>,
    token: &str,
    territory_code: &str,
) -> Result<Invitation> {
    let table_name = format!("territory_{}.invitation_invitations_tokens", territory_code);

    let query = format!(
        r#"
        SELECT id, token, created_by, max_uses, uses_count, expires_at,
               is_active, revoked_at, revoked_by, metadata, created_at, updated_at
        FROM {}
        WHERE token = $1
          AND is_active = true
          AND (expires_at IS NULL OR expires_at > NOW())
          AND (max_uses = 0 OR uses_count < max_uses)
        FOR UPDATE
        "#,
        table_name
    );

    let invitation = sqlx::query_as::<_, Invitation>(&query)
        .bind(token)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or_else(|| {
            AppError::NotFound("Invalid, expired, or fully used invitation token".to_string())
        })?;

    Ok(invitation)
}

/// Check if user has already used this invitation
async fn check_duplicate_usage(
    tx: &mut Transaction<'_, Postgres>,
    invitation_id: Uuid,
    used_by: Uuid,
    territory_code: &str,
) -> Result<()> {
    let table_name = format!("territory_{}.invitation_invitations_uses", territory_code);

    let query = format!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM {}
            WHERE invitation_id = $1 AND used_by = $2
        )
        "#,
        table_name
    );

    let already_used = sqlx::query_scalar::<_, bool>(&query)
        .bind(invitation_id)
        .bind(used_by)
        .fetch_one(&mut **tx)
        .await?;

    if already_used {
        return Err(AppError::Conflict(
            "User has already used this invitation token".to_string(),
        ));
    }

    Ok(())
}

/// Insert usage record into invitation_invitations_uses
async fn insert_invitation_use(
    tx: &mut Transaction<'_, Postgres>,
    invitation_id: Uuid,
    used_by: Uuid,
    territory_code: &str,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<()> {
    let table_name = format!("territory_{}.invitation_invitations_uses", territory_code);

    let query = format!(
        r#"
        INSERT INTO {} (invitation_id, used_by, ip_address, user_agent)
        VALUES ($1, $2, $3::inet, $4)
        "#,
        table_name
    );

    sqlx::query(&query)
        .bind(invitation_id)
        .bind(used_by)
        .bind(ip_address)
        .bind(user_agent)
        .execute(&mut **tx)
        .await?;

    Ok(())
}

/// Update invitation token usage count and active status
async fn update_invitation_usage(
    tx: &mut Transaction<'_, Postgres>,
    invitation_id: Uuid,
    max_uses: i32,
    territory_code: &str,
) -> Result<(i32, bool)> {
    let table_name = format!("territory_{}.invitation_invitations_tokens", territory_code);

    let query = format!(
        r#"
        UPDATE {}
        SET uses_count = uses_count + 1,
            is_active = CASE
                WHEN max_uses = 0 THEN true
                WHEN uses_count + 1 >= max_uses THEN false
                ELSE true
            END,
            updated_at = NOW()
        WHERE id = $1
        RETURNING uses_count, is_active
        "#,
        table_name
    );

    let row: (i32, bool) = sqlx::query_as(&query)
        .bind(invitation_id)
        .fetch_one(&mut **tx)
        .await?;

    let (uses_count, is_active) = row;
    let uses_remaining = if max_uses == 0 {
        -1 // Unlimited
    } else {
        max_uses - uses_count
    };

    // Fully used = not active (is_active set to false when uses_count >= max_uses)
    let fully_used = !is_active;

    Ok((uses_remaining, fully_used))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token_format() {
        let token = generate_token();

        // Check length (16 chars + 3 dashes = 19)
        assert_eq!(token.len(), 19);

        // Check format XXXX-XXXX-XXXX-XXXX
        let parts: Vec<&str> = token.split('-').collect();
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0].len(), 4);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
    }

    #[test]
    fn test_generate_token_no_confusing_chars() {
        // Generate 100 tokens to test randomness
        for _ in 0..100 {
            let token = generate_token();

            // Should not contain confusing characters
            assert!(!token.contains('0'), "Token contains 0: {}", token);
            assert!(!token.contains('O'), "Token contains O: {}", token);
            assert!(!token.contains('I'), "Token contains I: {}", token);
            assert!(!token.contains('1'), "Token contains 1: {}", token);
            assert!(!token.contains('l'), "Token contains l: {}", token);
        }
    }

    #[test]
    fn test_generate_token_uniqueness() {
        // Generate 1000 tokens and check for duplicates
        let mut tokens = std::collections::HashSet::new();

        for _ in 0..1000 {
            let token = generate_token();
            assert!(tokens.insert(token), "Duplicate token generated");
        }
    }
}

/// List invitations created by a user with filtering and pagination
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `user_id` - User ID to list invitations for
/// * `territory_code` - Territory code (e.g., "dk")
/// * `status_filter` - Optional status filter (active, used, expired, revoked)
/// * `page` - Page number (1-indexed)
/// * `limit` - Results per page (max 100)
///
/// # Returns
/// List of invitations with pagination info
pub async fn list_user_invitations(
    pool: &PgPool,
    user_id: Uuid,
    territory_code: &str,
    status_filter: Option<String>,
    page: i64,
    limit: i64,
) -> Result<ListInvitationsResponse> {
    // Clamp limit to max 100
    let limit = limit.min(100);
    let offset = (page - 1) * limit;

    // Build status filter SQL
    let status_condition = match status_filter.as_deref() {
        Some("active") => "AND is_active = true AND (expires_at IS NULL OR expires_at > NOW())",
        Some("used") => "AND uses_count >= max_uses AND max_uses > 0",
        Some("expired") => "AND expires_at IS NOT NULL AND expires_at <= NOW()",
        Some("revoked") => "AND revoked_at IS NOT NULL",
        _ => "", // No filter
    };

    // Query invitations
    let query = format!(
        r#"
        SELECT 
            id, token, max_uses, uses_count, is_active, 
            expires_at, created_at, revoked_at
        FROM territory_{}.invitation_invitations_tokens
        WHERE created_by = $1 {}
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        territory_code, status_condition
    );

    let rows = sqlx::query_as::<
        _,
        (
            Uuid,
            String,
            i32,
            i32,
            bool,
            Option<chrono::DateTime<Utc>>,
            chrono::DateTime<Utc>,
            Option<chrono::DateTime<Utc>>,
        ),
    >(&query)
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    // Get total count
    let count_query = format!(
        r#"
        SELECT COUNT(*)
        FROM territory_{}.invitation_invitations_tokens
        WHERE created_by = $1 {}
        "#,
        territory_code, status_condition
    );

    let total: i64 = sqlx::query_scalar(&count_query)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    // Build response with uses for each invitation
    let mut invitations = Vec::new();

    for (id, token, max_uses, uses_count, is_active, expires_at, created_at, revoked_at) in rows {
        // Get uses for this invitation
        let uses_query = format!(
            r#"
            SELECT u.used_by, a.username, u.used_at, u.ip_address
            FROM territory_{}.invitation_invitations_uses u
            JOIN territory_{}.auth_users_core a ON u.used_by = a.id
            WHERE u.invitation_id = $1
            ORDER BY u.used_at DESC
            "#,
            territory_code, territory_code
        );

        let uses: Vec<InvitationUse> =
            sqlx::query_as::<_, (Uuid, String, chrono::DateTime<Utc>, Option<String>)>(&uses_query)
                .bind(id)
                .fetch_all(pool)
                .await?
                .into_iter()
                .map(|(user_id, username, used_at, ip_address)| InvitationUse {
                    user_id,
                    username,
                    used_at,
                    ip_address,
                })
                .collect();

        // Determine status
        let status = if revoked_at.is_some() {
            "revoked".to_string()
        } else if let Some(exp) = expires_at {
            if exp <= Utc::now() {
                "expired".to_string()
            } else if max_uses > 0 && uses_count >= max_uses {
                "used".to_string()
            } else if is_active {
                "active".to_string()
            } else {
                "inactive".to_string()
            }
        } else if max_uses > 0 && uses_count >= max_uses {
            "used".to_string()
        } else if is_active {
            "active".to_string()
        } else {
            "inactive".to_string()
        };

        invitations.push(InvitationWithUses {
            id,
            token,
            max_uses,
            uses_count,
            is_active,
            expires_at,
            created_at,
            revoked_at,
            status,
            uses,
        });
    }

    Ok(ListInvitationsResponse {
        invitations,
        pagination: PaginationInfo { page, limit, total },
    })
}

/// Get detailed uses for a specific invitation
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `invitation_id` - Invitation ID to get uses for
/// * `territory_code` - Territory code (e.g., "dk")
/// * `requesting_user_id` - User ID making the request (for authorization)
///
/// # Returns
/// Invitation uses with details
///
/// # Authorization
/// Only the creator or a manager can view invitation uses
pub async fn get_invitation_uses(
    pool: &PgPool,
    invitation_id: Uuid,
    territory_code: &str,
    requesting_user_id: Uuid,
) -> Result<GetInvitationUsesResponse> {
    // Get invitation details and verify ownership
    let invitation_query = format!(
        r#"
        SELECT id, token, created_by, max_uses, uses_count
        FROM territory_{}.invitation_invitations_tokens
        WHERE id = $1
        "#,
        territory_code
    );

    let invitation: Option<(Uuid, String, Uuid, i32, i32)> = sqlx::query_as(&invitation_query)
        .bind(invitation_id)
        .fetch_optional(pool)
        .await?;

    let Some((id, token, created_by, max_uses, uses_count)) = invitation else {
        return Err(AppError::NotFound("Invitation not found".to_string()));
    };

    // Check if user is creator or manager
    if created_by != requesting_user_id {
        // Check if requesting user is a manager
        let is_manager = crate::services::permissions::check_manager_permissions(
            pool,
            requesting_user_id,
            territory_code,
        )
        .await?;

        if !is_manager {
            return Err(AppError::Forbidden(
                "You can only view uses for your own invitations".to_string(),
            ));
        }
    }

    // Get uses
    let uses_query = format!(
        r#"
        SELECT u.used_by, a.username, u.used_at, u.ip_address
        FROM territory_{}.invitation_invitations_uses u
        JOIN territory_{}.auth_users_core a ON u.used_by = a.id
        WHERE u.invitation_id = $1
        ORDER BY u.used_at DESC
        "#,
        territory_code, territory_code
    );

    let uses: Vec<InvitationUse> =
        sqlx::query_as::<_, (Uuid, String, chrono::DateTime<Utc>, Option<String>)>(&uses_query)
            .bind(invitation_id)
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|(user_id, username, used_at, ip_address)| InvitationUse {
                user_id,
                username,
                used_at,
                ip_address,
            })
            .collect();

    Ok(GetInvitationUsesResponse {
        invitation_id: id,
        token,
        uses,
        total_uses: uses_count,
        max_uses,
    })
}

/// Revoke an invitation
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `invitation_id` - Invitation ID to revoke
/// * `territory_code` - Territory code (e.g., "dk")
/// * `revoked_by` - User ID revoking the invitation
///
/// # Returns
/// Revocation confirmation
///
/// # Authorization
/// Only the creator or a manager can revoke an invitation
pub async fn revoke_invitation(
    pool: &PgPool,
    invitation_id: Uuid,
    territory_code: &str,
    revoked_by: Uuid,
) -> Result<RevokeInvitationResponse> {
    // Get invitation details and verify ownership
    let invitation_query = format!(
        r#"
        SELECT id, created_by, is_active, revoked_at
        FROM territory_{}.invitation_invitations_tokens
        WHERE id = $1
        "#,
        territory_code
    );

    let invitation: Option<(Uuid, Uuid, bool, Option<chrono::DateTime<Utc>>)> =
        sqlx::query_as(&invitation_query)
            .bind(invitation_id)
            .fetch_optional(pool)
            .await?;

    let Some((id, created_by, _is_active, revoked_at)) = invitation else {
        return Err(AppError::NotFound("Invitation not found".to_string()));
    };

    // Check if already revoked
    if revoked_at.is_some() {
        return Err(AppError::Conflict(
            "Invitation is already revoked".to_string(),
        ));
    }

    // Check if user is creator or manager
    if created_by != revoked_by {
        // Check if requesting user is a manager
        let is_manager = crate::services::permissions::check_manager_permissions(
            pool,
            revoked_by,
            territory_code,
        )
        .await?;

        if !is_manager {
            return Err(AppError::Forbidden(
                "You can only revoke your own invitations".to_string(),
            ));
        }
    }

    // Revoke invitation
    let revoked_at = Utc::now();
    let update_query = format!(
        r#"
        UPDATE territory_{}.invitation_invitations_tokens
        SET is_active = false, revoked_at = $1, revoked_by = $2
        WHERE id = $3
        "#,
        territory_code
    );

    sqlx::query(&update_query)
        .bind(revoked_at)
        .bind(revoked_by)
        .bind(invitation_id)
        .execute(pool)
        .await?;

    Ok(RevokeInvitationResponse {
        invitation_id: id,
        revoked_at,
    })
}
