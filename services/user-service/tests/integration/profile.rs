use crate::common::TestContext;
use user_service::models::profile::UpdateProfileRequest;
use user_service::services::UserService;

/// Helper to create an empty UpdateProfileRequest
fn empty_profile_request() -> UpdateProfileRequest {
    UpdateProfileRequest {
        about: None,
        interests: None,
        skills: None,
        languages: None,
        location: None,
        website_url: None,
        github_url: None,
        linkedin_url: None,
        twitter_handle: None,
        theme: None,
        metadata: None,
        profile_visibility: None,
        show_email: None,
        show_real_name: None,
        allow_messages_from: None,
    }
}

#[tokio::test]
async fn test_get_profile_nonexistent() {
    let ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    let fake_id = uuid::Uuid::new_v4();
    
    let result = service.get_profile(fake_id).await.expect("Query should succeed");
    
    assert!(result.is_none(), "Should return None for nonexistent user");
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_create_and_get_profile() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    // Create test user
    let user_id = ctx.create_user("testuser", "test@example.com").await;
    
    // Create profile
    let update_request = UpdateProfileRequest {
        about: Some(Some("Test bio".to_string())),
        interests: Some(vec!["Rust".to_string(), "Testing".to_string()]),
        skills: Some(vec!["Programming".to_string()]),
        languages: Some(vec!["English".to_string()]),
        location: Some(Some("Copenhagen".to_string())),
        website_url: Some(Some("https://example.com".to_string())),
        github_url: Some(None),
        linkedin_url: Some(None),
        twitter_handle: Some(None),
        theme: Some("dark".to_string()),
        metadata: Some(serde_json::json!({"custom": "data"})),
        profile_visibility: Some("public".to_string()),
        show_email: Some(false),
        show_real_name: Some(true),
        allow_messages_from: Some("everyone".to_string()),
    };
    
    let profile = service.update_profile(user_id, update_request).await
        .expect("Profile creation should succeed");
    
    // Verify profile was created
    assert_eq!(profile.user_id, user_id);
    assert_eq!(profile.about, Some("Test bio".to_string()));
    assert_eq!(profile.interests, Some(vec!["Rust".to_string(), "Testing".to_string()]));
    
    // Retrieve and verify
    let retrieved = service.get_profile(user_id).await
        .expect("Query should succeed")
        .expect("Profile should exist");
    
    assert_eq!(retrieved.user_id, user_id);
    assert_eq!(retrieved.about, Some("Test bio".to_string()));
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_update_profile_partial() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let user_id = ctx.create_user("partialuser", "partial@example.com").await;
    
    // Create initial profile
    let initial_request = UpdateProfileRequest {
        about: Some(Some("Initial bio".to_string())),
        interests: Some(vec!["Initial".to_string()]),
        ..empty_profile_request()
    };
    
    service.update_profile(user_id, initial_request).await
        .expect("Initial profile creation should succeed");
    
    // Partial update - only change location
    let update_request = UpdateProfileRequest {
        location: Some(Some("Aarhus".to_string())),
        ..empty_profile_request()
    };
    
    let updated = service.update_profile(user_id, update_request).await
        .expect("Profile update should succeed");
    
    assert_eq!(updated.location, Some("Aarhus".to_string()));
    assert_eq!(updated.about, Some("Initial bio".to_string()), "Other fields should remain unchanged");
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_public_profile_privacy_public() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let user_id = ctx.create_user("publicuser", "public@example.com").await;
    
    // Create profile with public visibility
    let request = UpdateProfileRequest {
        about: Some(Some("Public bio".to_string())),
        profile_visibility: Some("public".to_string()),
        ..empty_profile_request()
    };
    
    service.update_profile(user_id, request).await
        .expect("Profile creation should succeed");
    
    // Fetch as stranger
    let public_profile = service.get_public_profile(user_id, None).await
        .expect("Query should succeed")
        .expect("Profile should be visible");
    
    assert_eq!(public_profile.about, Some("Public bio".to_string()));
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_public_profile_privacy_private() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let user_id = ctx.create_user("privateuser", "private@example.com").await;
    
    // Create profile with private visibility
    let request = UpdateProfileRequest {
        about: Some(Some("Private bio".to_string())),
        profile_visibility: Some("private".to_string()),
        ..empty_profile_request()
    };
    
    service.update_profile(user_id, request).await
        .expect("Profile creation should succeed");
    
    // Fetch as stranger - should get None
    let public_profile = service.get_public_profile(user_id, None).await
        .expect("Query should succeed");
    
    assert!(public_profile.is_none(), "Private profile should not be visible to strangers");
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_public_profile_privacy_connections() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let user_id = ctx.create_user("connuser", "conn@example.com").await;
    let viewer_id = ctx.create_user("viewer", "viewer@example.com").await;
    let stranger_id = ctx.create_user("stranger", "stranger@example.com").await;
    
    // Create profile with connections-only visibility
    let request = UpdateProfileRequest {
        about: Some(Some("Connections-only bio".to_string())),
        profile_visibility: Some("connections".to_string()),
        ..empty_profile_request()
    };
    
    service.update_profile(user_id, request).await
        .expect("Profile creation should succeed");
    
    // Create connection (viewer follows user)
    sqlx::query(
        "INSERT INTO territory.user_connections (follower_id, following_id) VALUES ($1, $2)"
    )
    .bind(viewer_id)
    .bind(user_id)
    .execute(&ctx.pool)
    .await
    .expect("Connection creation should succeed");
    
    // Fetch as connection - should see profile
    let viewer_result = service.get_public_profile(user_id, Some(viewer_id)).await
        .expect("Query should succeed");
    
    assert!(viewer_result.is_some(), "Connected user should see profile");
    
    // Fetch as stranger - should get None
    let stranger_result = service.get_public_profile(user_id, Some(stranger_id)).await
        .expect("Query should succeed");
    
    assert!(stranger_result.is_none(), "Stranger should not see connections-only profile");
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_profile_show_email_privacy() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let user_id = ctx.create_user("emailuser", "email@example.com").await;
    
    // Create profile with email hidden
    let request = UpdateProfileRequest {
        show_email: Some(false),
        profile_visibility: Some("public".to_string()),
        ..empty_profile_request()
    };
    
    service.update_profile(user_id, request).await
        .expect("Profile creation should succeed");
    
    // Fetch public profile - email should be None
    let public_profile = service.get_public_profile(user_id, None).await
        .expect("Query should succeed")
        .expect("Profile should exist");
    
    assert!(public_profile.email.is_none(), "Email should be hidden in public profile");
    
    // Now enable email visibility
    let update = UpdateProfileRequest {
        show_email: Some(true),
        ..empty_profile_request()
    };
    
    service.update_profile(user_id, update).await
        .expect("Update should succeed");
    
    // Fetch again - email should be visible
    let updated_profile = service.get_public_profile(user_id, None).await
        .expect("Query should succeed")
        .expect("Profile should exist");
    
    assert!(updated_profile.email.is_some(), "Email should be visible");
    assert!(updated_profile.email.unwrap().contains("email"), "Email should contain 'email'");
    
    ctx.cleanup().await;
}
