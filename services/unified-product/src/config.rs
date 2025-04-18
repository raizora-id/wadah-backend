use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database_url: String,
    pub redis_url: String,
    pub core_platform_url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::default();

        // Add configuration values from a file named `config.toml`.
        cfg.merge(config::File::with_name("config").required(false))?;

        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        cfg.merge(config::Environment::with_prefix("APP").separator("__"))?;

        // Set default values
        cfg.set_default("server.host", "0.0.0.0")?;
        cfg.set_default("server.port", 8001)?;
        cfg.set_default("database_url", "postgres://postgres:postgres@localhost/klola_platform")?;
        cfg.set_default("redis_url", "redis://localhost:6379")?;
        cfg.set_default("core_platform_url", "http://localhost:8000")?;

        // Try to convert the configuration to our Config struct
        cfg.try_into()
    }
}