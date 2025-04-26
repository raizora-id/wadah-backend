use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub minio: MinioConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct MinioConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub use_ssl: bool,
}

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: String,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        cfg.try_deserialize()
    }
}
