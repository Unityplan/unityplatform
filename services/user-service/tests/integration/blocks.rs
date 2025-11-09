use crate::common::TestContext;
use user_service::services::UserService;

#[tokio::test]
async fn test_block_user() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let blocker_id = ctx.create_user("blocker", "blocker@example.com").await;
    let blocked_id = ctx.create_user("blocked", "blocked@example.com").await;
    
    service.block_user(blocker_id, blocked_id, Some("Spam".to_string())).await
        .expect("Block should succeed");
    
    // Verify block exists
    let blocked_users = service.get_blocked_users(blocker_id).await
        .expect("Query should succeed");
    
    assert_eq!(blocked_users.len(), 1);
    assert_eq!(blocked_users[0].blocked_id, blocked_id);
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_unblock_user() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let blocker_id = ctx.create_user("blocker", "blocker@example.com").await;
    let blocked_id = ctx.create_user("blocked", "blocked@example.com").await;
    
    // Block user
    service.block_user(blocker_id, blocked_id, Some("Testing".to_string())).await
        .expect("Block should succeed");
    
    // Unblock
    service.unblock_user(blocker_id, blocked_id).await
        .expect("Unblock should succeed");
    
    // Verify block removed
    let blocked_users = service.get_blocked_users(blocker_id).await
        .expect("Query should succeed");
    
    assert_eq!(blocked_users.len(), 0, "Blocked list should be empty");
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_blocking_removes_connections() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let user1_id = ctx.create_user("user1", "user1@example.com").await;
    let user2_id = ctx.create_user("user2", "user2@example.com").await;
    
    // Create mutual following
    service.follow_user(user1_id, user2_id).await.expect("Follow should succeed");
    service.follow_user(user2_id, user1_id).await.expect("Follow should succeed");
    
    // Verify connections exist
    let user1_following = service.get_following(user1_id).await
        .expect("Query should succeed");
    assert_eq!(user1_following.len(), 1, "Should have one following before block");
    
    // User1 blocks User2
    service.block_user(user1_id, user2_id, Some("Not interested".to_string())).await
        .expect("Block should succeed");
    
    // Verify connections are removed
    let user1_following_after = service.get_following(user1_id).await
        .expect("Query should succeed");
    let user2_following_after = service.get_following(user2_id).await
        .expect("Query should succeed");
    
    assert_eq!(user1_following_after.len(), 0, "Blocker's following should be empty");
    assert_eq!(user2_following_after.len(), 0, "Blocked user's following should be empty");
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_get_blocked_users() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let blocker_id = ctx.create_user("blocker", "blocker@example.com").await;
    let blocked1_id = ctx.create_user("blocked1", "blocked1@example.com").await;
    let blocked2_id = ctx.create_user("blocked2", "blocked2@example.com").await;
    
    service.block_user(blocker_id, blocked1_id, Some("Reason 1".to_string())).await
        .expect("Block should succeed");
    service.block_user(blocker_id, blocked2_id, Some("Reason 2".to_string())).await
        .expect("Block should succeed");
    
    let blocked_users = service.get_blocked_users(blocker_id).await
        .expect("Query should succeed");
    
    assert_eq!(blocked_users.len(), 2);
    
    let blocked_ids: Vec<_> = blocked_users.iter().map(|u| u.blocked_id).collect();
    assert!(blocked_ids.contains(&blocked1_id));
    assert!(blocked_ids.contains(&blocked2_id));
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_mutual_blocks() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let user1_id = ctx.create_user("user1", "user1@example.com").await;
    let user2_id = ctx.create_user("user2", "user2@example.com").await;
    
    // Both users block each other
    service.block_user(user1_id, user2_id, Some("Mutual dislike".to_string())).await
        .expect("Block should succeed");
    service.block_user(user2_id, user1_id, Some("Mutual dislike".to_string())).await
        .expect("Block should succeed");
    
    // Verify both blocks exist
    let user1_blocked = service.get_blocked_users(user1_id).await
        .expect("Query should succeed");
    let user2_blocked = service.get_blocked_users(user2_id).await
        .expect("Query should succeed");
    
    assert_eq!(user1_blocked.len(), 1);
    assert_eq!(user1_blocked[0].blocked_id, user2_id);
    
    assert_eq!(user2_blocked.len(), 1);
    assert_eq!(user2_blocked[0].blocked_id, user1_id);
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_update_block_reason() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let blocker_id = ctx.create_user("blocker", "blocker@example.com").await;
    let blocked_id = ctx.create_user("blocked", "blocked@example.com").await;
    
    // Initial block
    service.block_user(blocker_id, blocked_id, Some("Initial reason".to_string())).await
        .expect("Block should succeed");
    
    // Update reason by blocking again (ON CONFLICT DO UPDATE)
    service.block_user(blocker_id, blocked_id, Some("Updated reason".to_string())).await
        .expect("Block update should succeed");
    
    // Verify only one block exists
    let blocked_users = service.get_blocked_users(blocker_id).await
        .expect("Query should succeed");
    
    assert_eq!(blocked_users.len(), 1, "Should have exactly one block");
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_empty_blocked_list() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let user_id = ctx.create_user("peaceful", "peaceful@example.com").await;
    
    let blocked_users = service.get_blocked_users(user_id).await
        .expect("Query should succeed");
    
    assert_eq!(blocked_users.len(), 0, "New user should have no blocks");
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_block_prevents_follow() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let blocker_id = ctx.create_user("blocker", "blocker@example.com").await;
    let blocked_id = ctx.create_user("blocked", "blocked@example.com").await;
    
    // Block user first
    service.block_user(blocker_id, blocked_id, Some("Don't want to interact".to_string())).await
        .expect("Block should succeed");
    
    // Try to follow blocked user - should fail
    let follow_result = service.follow_user(blocker_id, blocked_id).await;
    
    assert!(follow_result.is_err(), "Following a blocked user should fail");
    
    // Try reverse follow (blocked user follows blocker) - should also fail
    let reverse_follow_result = service.follow_user(blocked_id, blocker_id).await;
    
    assert!(reverse_follow_result.is_err(), "Blocked user should not be able to follow blocker");
    
    ctx.cleanup().await;
}
