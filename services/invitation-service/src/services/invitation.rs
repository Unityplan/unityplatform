use chrono::{Duration, Utc};
use rand::Rng;
use shared_lib::{AppError, Result};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::models::invitation::Invitation;

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

    Ok(invitation)
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
