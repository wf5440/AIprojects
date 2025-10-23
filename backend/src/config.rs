use serde::Deserialize;
use config::{Config, ConfigError, File, Environment};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub log_level: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name("Config/default").required(false))
            .add_source(Environment::default().separator("__"));
            
        builder.build()?.try_deserialize()
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://user:pass@localhost:5432/db".to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            jwt_secret: "default_secret".to_string(),
            port: 8080,
            log_level: "info".to_string(),
        }
    }
}