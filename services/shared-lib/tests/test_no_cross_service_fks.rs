//! Test to verify no cross-service foreign keys exist
//! 
//! This test queries PostgreSQL system catalog to detect any foreign keys
//! from non-auth services to auth_users_core table.

use sqlx::PgPool;

#[derive(Debug, sqlx::FromRow)]
struct ForeignKey {
    table_schema: String,
    table_name: String,
    column_name: String,
    foreign_table_name: String,
    foreign_column_name: String,
}

async fn get_test_pool() -> PgPool {
    // Use direct connection for testing
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://unityplatform:unityplatform_dev_password_dk@localhost:5432/unityplatform_dk".to_string());
    
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}

#[tokio::test]
async fn test_no_cross_service_foreign_keys_to_auth() {
    let pool = get_test_pool().await;
    
    let fks = sqlx::query_as::<_, ForeignKey>(
        "SELECT 
            tc.table_schema,
            tc.table_name,
            kcu.column_name,
            ccu.table_name AS foreign_table_name,
            ccu.column_name AS foreign_column_name
         FROM information_schema.table_constraints AS tc 
         JOIN information_schema.key_column_usage AS kcu
           ON tc.constraint_name = kcu.constraint_name
           AND tc.table_schema = kcu.table_schema
         JOIN information_schema.constraint_column_usage AS ccu
           ON ccu.constraint_name = tc.constraint_name
           AND ccu.table_schema = tc.table_schema
         WHERE tc.constraint_type = 'FOREIGN KEY'
           AND tc.table_schema = 'territory_dk'
           AND ccu.table_name = 'auth_users_core'
           AND tc.table_name NOT LIKE 'auth_%'
         ORDER BY tc.table_name, kcu.column_name"
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to query foreign keys");
    
    if !fks.is_empty() {
        eprintln!("\n❌ Found {} cross-service foreign key(s) to auth_users_core:", fks.len());
        for fk in &fks {
            eprintln!(
                "   - {}.{}.{} -> {}.{}",
                fk.table_schema, 
                fk.table_name, 
                fk.column_name,
                fk.foreign_table_name,
                fk.foreign_column_name
            );
        }
        eprintln!("\n💡 Services should validate via JWT tokens, not database FKs.");
        eprintln!("   See: docs/guides/development/FK-REMOVAL-IMPLEMENTATION.md");
        panic!("Cross-service foreign keys detected");
    }
    
    println!("✅ No cross-service foreign keys detected");
}

#[tokio::test]
async fn test_settings_service_has_no_fks() {
    let pool = get_test_pool().await;
    
    // Specific test for settings-service after FK removal
    let fks = sqlx::query_as::<_, ForeignKey>(
        "SELECT 
            tc.table_schema,
            tc.table_name,
            kcu.column_name,
            ccu.table_name AS foreign_table_name,
            ccu.column_name AS foreign_column_name
         FROM information_schema.table_constraints AS tc 
         JOIN information_schema.key_column_usage AS kcu
           ON tc.constraint_name = kcu.constraint_name
         JOIN information_schema.constraint_column_usage AS ccu
           ON ccu.constraint_name = tc.constraint_name
         WHERE tc.constraint_type = 'FOREIGN KEY'
           AND tc.table_schema = 'territory_dk'
           AND tc.table_name = 'settings_users_settings'
           AND ccu.table_name = 'auth_users_core'"
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to query foreign keys");
    
    assert_eq!(
        fks.len(), 
        0, 
        "settings_users_settings should have no FKs to auth_users_core"
    );
    
    println!("✅ settings-service has no cross-service FKs");
}
