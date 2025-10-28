use config::{Config as ConfigBuilder, ConfigError, Environment};
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub stellar: StellarConfig,
    pub database: DatabaseConfig,
    pub aa: AccountAbstractionConfig,
    pub reputation: ReputationConfig,
    pub external_apis: ExternalApisConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub frontend_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StellarConfig {
    pub network: String,
    pub horizon_url: String,
    pub friendbot_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountAbstractionConfig {
    pub bundler_url: String,
    pub signer_memory: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReputationConfig {
    pub threshold: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExternalApisConfig {
    pub circle_api_key: Option<String>,
    pub stripe_secret_key: Option<String>,
    pub coingecko_api_url: String,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let config = ConfigBuilder::builder()
            .set_default("server.port", 4000)?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.frontend_url", "http://localhost:3000")?
            .set_default("stellar.network", "testnet")?
            .set_default("stellar.horizon_url", "https://horizon-testnet.stellar.org")?
            .set_default("stellar.friendbot_url", "https://friendbot.stellar.org")?
            .set_default("database.url", "sqlite://./wallet.db")?
            .set_default("aa.bundler_url", "http://localhost:4100")?
            .set_default("aa.signer_memory", true)?
            .set_default("reputation.threshold", 50)?
            .set_default("external_apis.coingecko_api_url", "https://api.coingecko.com/api/v3")?
            .add_source(Environment::default().try_parsing(true))
            .build()?;

        config.try_deserialize()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.server.port == 0 {
            return Err("Invalid port configuration".to_string());
        }

        if self.stellar.horizon_url.is_empty() {
            return Err("Stellar Horizon URL is required".to_string());
        }

        if self.reputation.threshold > 100 {
            return Err("Reputation threshold must be between 0-100".to_string());
        }

        Ok(())
    }
}