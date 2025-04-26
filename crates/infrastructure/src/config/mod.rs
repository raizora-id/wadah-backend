use std::env;
use serde::Deserialize;
use config::{Config as ConfigBuilder, ConfigError, Environment, File};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub environment: Environment,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub minio: MinioConfig,
    pub jwt: JwtConfig,
    pub telemetry: TelemetryConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Staging => "staging",
            Environment::Production => "production",
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::Development
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub request_timeout: u64,
    pub graceful_shutdown_timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub max_lifetime: u64,
    pub idle_timeout: u64,
    pub connect_timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub pool_max_open: u32,
    pub pool_max_idle: u32,
    pub pool_timeout: u64,
    pub pool_expire: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MinioConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub region: String,
    pub use_ssl: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: String,
    pub refresh_expiration: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TelemetryConfig {
    pub log_level: String,
    pub jaeger_endpoint: Option<String>,
    pub datadog_endpoint: Option<String>,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let env = env::var("RUST_ENV").unwrap_or_else(|_| "development".into());
        let config_dir = env::var("CONFIG_DIR").unwrap_or_else(|_| "config".into());

        let builder = ConfigBuilder::builder()
            // Start with base config
            .add_source(File::with_name(&format!("{}/base", config_dir)).required(false))
            // Add environment specific config
            .add_source(File::with_name(&format!("{}/{}", config_dir, env)).required(false))
            // Add local config for development
            .add_source(File::with_name(&format!("{}/local", config_dir)).required(false))
            // Add environment variables with prefix "APP_"
            .add_source(
                Environment::with_prefix("APP")
                    .separator("_")
                    .try_parsing(true)
                    .list_separator(",")
            );

        // Build and deserialize
        builder.build()?.try_deserialize()
    }

    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    pub fn is_production(&self) -> bool {
        matches!(self.environment, Environment::Production)
    }
}
