use crate::common::TestContext;
use user_service::services::UserService;

#[tokio::test]
async fn test_follow_user() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let follower_id = ctx.create_user("follower", "follower@example.com").await;
    let following_id = ctx.create_user("following", "following@example.com").await;
    
    service.follow_user(follower_id, following_id).await
        .expect("Follow should succeed");
    
    // Verify connection exists
    let followers = service.get_followers(following_id).await
        .expect("Query should succeed");
    
    assert_eq!(followers.len(), 1);
    assert_eq!(followers[0].user_id, follower_id);
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_unfollow_user() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let follower_id = ctx.create_user("follower", "follower@example.com").await;
    let following_id = ctx.create_user("following", "following@example.com").await;
    
    // Create connection
    service.follow_user(follower_id, following_id).await
        .expect("Follow should succeed");
    
    // Unfollow
    service.unfollow_user(follower_id, following_id).await
        .expect("Unfollow should succeed");
    
    // Verify connection removed
    let followers = service.get_followers(following_id).await
        .expect("Query should succeed");
    
    assert_eq!(followers.len(), 0, "Follower list should be empty");
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_get_followers() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let user_id = ctx.create_user("popular", "popular@example.com").await;
    let follower1_id = ctx.create_user("follower1", "follower1@example.com").await;
    let follower2_id = ctx.create_user("follower2", "follower2@example.com").await;
    
    service.follow_user(follower1_id, user_id).await.expect("Follow should succeed");
    service.follow_user(follower2_id, user_id).await.expect("Follow should succeed");
    
    let followers = service.get_followers(user_id).await
        .expect("Query should succeed");
    
    assert_eq!(followers.len(), 2);
    
    let follower_ids: Vec<_> = followers.iter().map(|u| u.user_id).collect();
    assert!(follower_ids.contains(&follower1_id));
    assert!(follower_ids.contains(&follower2_id));
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_get_following() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let user_id = ctx.create_user("active", "active@example.com").await;
    let following1_id = ctx.create_user("following1", "following1@example.com").await;
    let following2_id = ctx.create_user("following2", "following2@example.com").await;
    
    service.follow_user(user_id, following1_id).await.expect("Follow should succeed");
    service.follow_user(user_id, following2_id).await.expect("Follow should succeed");
    
    let following = service.get_following(user_id).await
        .expect("Query should succeed");
    
    assert_eq!(following.len(), 2);
    
    let following_ids: Vec<_> = following.iter().map(|u| u.user_id).collect();
    assert!(following_ids.contains(&following1_id));
    assert!(following_ids.contains(&following2_id));
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_mutual_following() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let user1_id = ctx.create_user("user1", "user1@example.com").await;
    let user2_id = ctx.create_user("user2", "user2@example.com").await;
    
    // User1 follows User2
    service.follow_user(user1_id, user2_id).await.expect("Follow should succeed");
    
    // User2 follows User1 back
    service.follow_user(user2_id, user1_id).await.expect("Follow should succeed");
    
    // Verify mutual following
    let user1_following = service.get_following(user1_id).await
        .expect("Query should succeed");
    let user2_following = service.get_following(user2_id).await
        .expect("Query should succeed");
    
    assert_eq!(user1_following.len(), 1);
    assert_eq!(user1_following[0].user_id, user2_id);
    
    assert_eq!(user2_following.len(), 1);
    assert_eq!(user2_following[0].user_id, user1_id);
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_duplicate_follow() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let follower_id = ctx.create_user("follower", "follower@example.com").await;
    let following_id = ctx.create_user("following", "following@example.com").await;
    
    // First follow
    service.follow_user(follower_id, following_id).await
        .expect("First follow should succeed");
    
    // Duplicate follow - should handle gracefully (ON CONFLICT DO NOTHING)
    service.follow_user(follower_id, following_id).await
        .expect("Duplicate follow should not error");
    
    // Should still have only one connection
    let followers = service.get_followers(following_id).await
        .expect("Query should succeed");
    
    assert_eq!(followers.len(), 1);
    
    ctx.cleanup().await;
}

#[tokio::test]
async fn test_empty_followers_and_following() {
    let mut ctx = TestContext::new().await;
    let service = UserService::new(ctx.pool.clone());
    
    let user_id = ctx.create_user("lonely", "lonely@example.com").await;
    
    let followers = service.get_followers(user_id).await
        .expect("Query should succeed");
    let following = service.get_following(user_id).await
        .expect("Query should succeed");
    
    assert_eq!(followers.len(), 0, "New user should have no followers");
    assert_eq!(following.len(), 0, "New user should follow nobody");
    
    ctx.cleanup().await;
}
