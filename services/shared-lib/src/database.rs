use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use crate::error::Result;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str, max_connections: u32, min_connections: u32) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .min_connections(min_connections)
            .acquire_timeout(Duration::from_secs(5))
            .connect(database_url)
            .await?;

        tracing::info!(
            "Database connection pool established (min: {}, max: {})",
            min_connections,
            max_connections
        );

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn ping(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn close(self) {
        self.pool.close().await;
        tracing::info!("Database connection pool closed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires database to be running
    async fn test_database_connection() {
        let db = Database::new(
            "postgres://unityplan:unityplan_dev_password_dk@localhost:5432/unityplan_dk",
            5,
            2
        ).await.unwrap();

        assert!(db.ping().await.is_ok());
        db.close().await;
    }
}
