use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub territory: TerritoryConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub access_ttl: i64,  // seconds
    pub refresh_ttl: i64, // seconds
}

#[derive(Debug, Clone, Deserialize)]
pub struct TerritoryConfig {
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database: DatabaseConfig {
                url: std::env::var("DATABASE_URL").unwrap_or_else(|_| {
                    "postgresql://unityplatform:unityplatform_dev_password_dk@localhost:5432/unityplatform"
                        .to_string()
                }),
            },
            server: ServerConfig {
                host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: std::env::var("SERVER_PORT")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(8010),
            },
            jwt: JwtConfig {
                secret: std::env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "dev_secret_change_in_production".to_string()),
                access_ttl: std::env::var("JWT_ACCESS_TTL")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(900), // 15 minutes
                refresh_ttl: std::env::var("JWT_REFRESH_TTL")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(604800), // 7 days
            },
            territory: TerritoryConfig {
                code: std::env::var("TERRITORY_CODE").unwrap_or_else(|_| "dk".to_string()),
            },
            cors: CorsConfig {
                allowed_origins: std::env::var("CORS_ALLOWED_ORIGINS")
                    .unwrap_or_else(|_| "http://localhost:5173,http://localhost:3000".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect(),
            },
        })
    }

    pub fn get_territory_schema(&self) -> String {
        format!("territory_{}", self.territory.code)
    }
}
