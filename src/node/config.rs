use config::{Config, ConfigError, Environment, File};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub api_key: String,
    pub debug: bool,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok(); // Load environment variables from `.env` file, if it exists

        // Create a ConfigBuilder and add settings from files and environment variables
        let builder = Config::builder()
            .add_source(File::with_name("config/default")) // Load default configuration file
            .add_source(Environment::with_prefix("APP")); // Add environment variables with prefix APP_

        // Build the configuration and convert it into the AppConfig struct
        builder.build()?.try_deserialize()
    }
}
