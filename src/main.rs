use clap::Parser;
use std::error::Error;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

mod node;
use node::blockchain::Blockchain;
use node::config::AppConfig;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "default")]
    env: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let env = &args.env;
    let config = load_configuration(env)?;
    setup_tracing(&config.log_level)?;

    let blockchain = initialize_blockchain(&config);
    blockchain.start_network_services(&config).await;
    Ok(())
}

fn load_configuration(env: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config = AppConfig::from_env(env)?;
    println!("Loaded configuration from env {:?}: {:?}", env, config);
    info!("test info from mehran");
    warn!("test warn from mehran");
    Ok(config)
}

fn initialize_blockchain(config: &AppConfig) -> Blockchain {
    Blockchain::new(
        config.blockchain_name.clone(),
        config.author_public_key.clone(),
        config.author_secret_key.clone(),
        config.developer_mode.clone(),
        config.authorities.clone(),
    )
}

fn setup_tracing(log_level: &str) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_level))
        .try_init()
        .or_else(|_| {
            println!("Global default trace dispatcher has already been set");
            Ok::<(), Box<dyn Error>>(())
        })?;
    Ok(())
}