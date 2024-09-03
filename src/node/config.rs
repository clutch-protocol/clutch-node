use config::{Config, ConfigError, Environment, File};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub libp2p_topic_name: String,
    pub blockchain_name: String,
    pub author_public_key: String,
    pub author_secret_key: String,
    pub developer_mode: bool,
    pub websocket_addr: String,
    pub authorities: Vec<String>,
    pub listen_addrs: Vec<String>,
    pub bootstrap_nodes: Vec<String>,
    pub block_authoring_enabled: bool,
    pub sync_enabled: bool,
}

impl AppConfig {
    pub fn from_env(env: &str) -> Result<Self, ConfigError> {
        dotenv().ok();
        let file_path = format!("config/node/{}.toml", env);

        let builder = Config::builder()
            .add_source(File::with_name(&file_path))
            .add_source(Environment::with_prefix("APP"));

        builder.build()?.try_deserialize()
    }
}
