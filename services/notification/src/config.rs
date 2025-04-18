use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database_url: String,
    pub redis_url: String,
    pub email: EmailConfig,
    pub sms: SmsConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SmsConfig {
    pub provider: String,
    pub api_key: String,
    pub from_number: String,
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
        cfg.set_default("server.port", 8002)?;
        cfg.set_default("database_url", "postgres://postgres:postgres@localhost/klola_platform")?;
        cfg.set_default("redis_url", "redis://localhost:6379")?;
        
        // Default email config
        cfg.set_default("email.smtp_host", "smtp.example.com")?;
        cfg.set_default("email.smtp_port", 587)?;
        cfg.set_default("email.smtp_username", "user")?;
        cfg.set_default("email.smtp_password", "password")?;
        cfg.set_default("email.from_email", "notifications@example.com")?;
        cfg.set_default("email.from_name", "Klola Notifications")?;
        
        // Default SMS config
        cfg.set_default("sms.provider", "twilio")?;
        cfg.set_default("sms.api_key", "change_me_in_production")?;
        cfg.set_default("sms.from_number", "+1234567890")?;

        // Try to convert the configuration to our Config struct
        cfg.try_into()
    }
}