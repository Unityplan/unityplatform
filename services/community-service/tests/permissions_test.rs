use community_service_service::services::CommunityService;
use shared_lib::{AppConfig, Database};
use sqlx::{PgPool, Row};
use uuid::Uuid;

async fn setup_service() -> (CommunityService, PgPool) {
    dotenvy::from_filename(".env.test").ok();
    // Ensure DATABASE_URL is set for sqlx if we were using macros, but we'll switch to functions

    let config = AppConfig::from_env().expect("Failed to load config");
    let database = Database::new(
        &config.database.url,
        config.database.max_connections,
        config.database.min_connections,
    )
    .await
    .expect("Failed to connect to database");
    let pool = database.pool().clone();

    let service = CommunityService::new(pool.clone());

    (service, pool)
}

async fn create_user(pool: &PgPool, username: &str) -> Uuid {
    let user_id = Uuid::new_v4();

    // Insert into global registry first
    sqlx::query(
        "INSERT INTO global.registry_username (username, territory_code, user_id) VALUES ($1, 'dk', $2)",
    )
    .bind(username)
    .bind(user_id)
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO territory_dk.auth_users_core (id, username, password_hash, territory_code) VALUES ($1, $2, 'hash', 'dk')",
    )
    .bind(user_id)
    .bind(username)
    .execute(pool)
    .await
    .unwrap();
    user_id
}

async fn create_community(pool: &PgPool, name: &str, parent_id: Option<Uuid>) -> Uuid {
    let id = Uuid::new_v4();
    let slug = format!("{}-{}", name.to_lowercase(), Uuid::new_v4());
    sqlx::query(
        "INSERT INTO territory_dk.communities (id, slug, name, type, parent_community_id) VALUES ($1, $2, $3, 'zone', $4)",
    )
    .bind(id)
    .bind(slug)
    .bind(name)
    .bind(parent_id)
    .execute(pool)
    .await
    .unwrap();
    id
}

async fn assign_community_admin(pool: &PgPool, community_id: Uuid, user_id: Uuid) {
    sqlx::query(
        "INSERT INTO territory_dk.community_members (community_id, user_id, role) VALUES ($1, $2, 'admin')",
    )
    .bind(community_id)
    .bind(user_id)
    .execute(pool)
    .await
    .unwrap();

    // Also assign the community-manager badge
    assign_badge(pool, user_id, "community-manager").await;
}

async fn assign_badge(pool: &PgPool, user_id: Uuid, slug: &str) {
    // Ensure badge exists
    let badge_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO global.registry_badge (id, slug, name, description, icon, category, criteria_type) 
         VALUES ($1, $2, $3, 'desc', 'icon', 'role', 'manual')
         ON CONFLICT (slug) DO UPDATE SET name = EXCLUDED.name RETURNING id",
    )
    .bind(badge_id)
    .bind(slug)
    .bind(slug)
    .fetch_one(pool)
    .await
    .unwrap();

    // Get the actual ID (in case of conflict)
    let row = sqlx::query("SELECT id FROM global.registry_badge WHERE slug = $1")
        .bind(slug)
        .fetch_one(pool)
        .await
        .unwrap();

    let actual_badge_id: Uuid = row.get("id");

    sqlx::query("INSERT INTO territory_dk.badge_users_badges (user_id, badge_id) VALUES ($1, $2)")
        .bind(user_id)
        .bind(actual_badge_id)
        .execute(pool)
        .await
        .unwrap();
}

async fn assign_territory_manager_role(pool: &PgPool, user_id: Uuid) {
    sqlx::query(
        "INSERT INTO territory_dk.territory_territories_managers (user_id, territory_code) VALUES ($1, 'dk')",
    )
    .bind(user_id)
    .execute(pool)
    .await
    .unwrap();
}

#[actix_web::test]
async fn test_permission_hierarchy() {
    let (service, pool) = setup_service().await;

    // 1. Setup Users
    let suffix = Uuid::new_v4().simple();
    let child_admin = create_user(&pool, &format!("child_admin_{}", suffix)).await;
    let parent_admin = create_user(&pool, &format!("parent_admin_{}", suffix)).await;
    let grandparent_admin = create_user(&pool, &format!("grandparent_admin_{}", suffix)).await;
    let territory_manager = create_user(&pool, &format!("territory_manager_{}", suffix)).await;
    let platform_manager = create_user(&pool, &format!("platform_manager_{}", suffix)).await;
    let random_user = create_user(&pool, &format!("random_user_{}", suffix)).await;

    assign_badge(&pool, territory_manager, "territory-manager").await;
    assign_territory_manager_role(&pool, territory_manager).await;
    assign_badge(&pool, platform_manager, "platform-manager").await;

    // 2. Setup Communities (Grandparent -> Parent -> Child)
    let grandparent = create_community(&pool, "Grandparent", None).await;
    let parent = create_community(&pool, "Parent", Some(grandparent)).await;
    let child = create_community(&pool, "Child", Some(parent)).await;

    // 3. Test Scenario: No local admins
    // Only Territory Manager should have access (Distance 1000 vs Platform 2000)
    // Wait, if Territory Manager exists, Platform Manager is blocked.
    // But if we check for Platform Manager alone, they should have access.

    // Let's check Platform Manager first (assuming no Territory Manager is relevant yet? No, the query finds ALL managers)
    // The query finds ALL managers. So if both exist in the DB, both are returned.
    // But `is_community_admin` checks if the user is in the "closest" group.

    // If I check `is_community_admin(child, platform_manager)`, it fetches all managers.
    // The list will contain: Territory Manager (1000), Platform Manager (2000).
    // Closest is 1000.
    // Platform Manager is at 2000.
    // So Platform Manager should be DENIED if a Territory Manager exists in the system?
    // YES, that's what the code does. "Only allow managers at the minimum distance".
    // This implies that if a Territory Manager exists *anywhere* (since they are global), they trump Platform Managers.
    // This seems correct for the "closest" logic, but might be confusing if the Territory Manager is not "active" or "assigned" to this specific territory?
    // The query joins `territory_dk.badge_users_badges`. So it finds users who HAVE the badge in THIS territory schema.
    // So yes, if there is a Territory Manager in DK, they trump the Platform Manager for DK communities.

    assert!(
        service
            .is_community_admin(child, territory_manager)
            .await
            .unwrap(),
        "Territory Manager should have access when no local admins"
    );
    assert!(
        !service
            .is_community_admin(child, platform_manager)
            .await
            .unwrap(),
        "Platform Manager should be blocked by Territory Manager"
    );
    assert!(
        !service
            .is_community_admin(child, random_user)
            .await
            .unwrap(),
        "Random user should be denied"
    );

    // 4. Test Scenario: Grandparent Admin assigned
    assign_community_admin(&pool, grandparent, grandparent_admin).await;

    // Now closest is Grandparent Admin (Distance 2)
    // Territory (1000) and Platform (2000) should be blocked.
    assert!(
        service
            .is_community_admin(child, grandparent_admin)
            .await
            .unwrap(),
        "Grandparent Admin should have access"
    );
    assert!(
        !service
            .is_community_admin(child, territory_manager)
            .await
            .unwrap(),
        "Territory Manager should be blocked by Grandparent Admin"
    );

    // 5. Test Scenario: Parent Admin assigned
    assign_community_admin(&pool, parent, parent_admin).await;

    // Now closest is Parent Admin (Distance 1)
    assert!(
        service
            .is_community_admin(child, parent_admin)
            .await
            .unwrap(),
        "Parent Admin should have access"
    );
    assert!(
        !service
            .is_community_admin(child, grandparent_admin)
            .await
            .unwrap(),
        "Grandparent Admin should be blocked by Parent Admin"
    );

    // 6. Test Scenario: Child Admin assigned
    assign_community_admin(&pool, child, child_admin).await;

    // Now closest is Child Admin (Distance 0)
    assert!(
        service
            .is_community_admin(child, child_admin)
            .await
            .unwrap(),
        "Child Admin should have access"
    );
    assert!(
        !service
            .is_community_admin(child, parent_admin)
            .await
            .unwrap(),
        "Parent Admin should be blocked by Child Admin"
    );

    // 7. Verify get_effective_managers filtering
    // At this point, we have a Child Admin (distance 0).
    // get_effective_managers should return ONLY the Child Admin.
    let managers = service.get_effective_managers(child).await.unwrap();
    assert_eq!(
        managers.len(),
        1,
        "Should only have 1 effective manager (Child Admin)"
    );
    assert_eq!(managers[0].user_id, child_admin);
    assert_eq!(managers[0].distance, 0);
}
