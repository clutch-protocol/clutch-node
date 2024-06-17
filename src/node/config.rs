use config::{Config, ConfigError, Environment, File};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub libp2p_topic_name: String,
}

impl AppConfig {
    pub fn from_env(env: &str) -> Result<Self, ConfigError> {
        dotenv().ok(); // Load environment variables from `.env` file, if it exists

        // Construct the file path based on the environment name
        let file_path = format!("config/node/{}.toml", env);

        // Create a ConfigBuilder and add settings from the specified file and environment variables
        let builder = Config::builder()
            .add_source(File::with_name(&file_path)) // Load configuration from the specified environment file if it exists
            .add_source(Environment::with_prefix("APP")); // Add environment variables with prefix APP_

        // Build the configuration and convert it into the AppConfig struct
        builder.build()?.try_deserialize()
    }
}
