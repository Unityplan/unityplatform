use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub nats: NatsConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub pod_id: String,
    pub territory: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NatsConfig {
    pub url: String,
    pub cluster_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        // Load .env file if present
        dotenvy::dotenv().ok();

        let mut settings = config::Config::builder();

        // Set defaults
        settings = settings
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .set_default("database.max_connections", 20)?
            .set_default("database.min_connections", 5)?
            .set_default("nats.cluster_name", "unityplan-global")?
            .set_default("auth.jwt_expiration_hours", 24)?;

        // Override with environment variables
        settings = settings.add_source(
            config::Environment::with_prefix("APP")
                .separator("__")
                .try_parsing(true),
        );

        settings.build()?.try_deserialize()
    }

    pub fn database_url(&self) -> &str {
        &self.database.url
    }

    pub fn nats_url(&self) -> &str {
        &self.nats.url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_defaults() {
        env::set_var("APP__DATABASE__URL", "postgres://localhost/test");
        env::set_var("APP__NATS__URL", "nats://localhost:4222");
        env::set_var("APP__AUTH__JWT_SECRET", "test_secret");
        env::set_var("APP__SERVER__POD_ID", "dk");
        env::set_var("APP__SERVER__TERRITORY", "denmark");

        let config = AppConfig::from_env().unwrap();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.server.pod_id, "dk");
        assert_eq!(config.database.max_connections, 20);
    }
}
