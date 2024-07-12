use clap::Parser;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use node::blockchain::Blockchain;

mod node;

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
    let blockchain = Arc::new(Mutex::new(initialize_blockchain(&config)));
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    node::network::Network::start_services(&config, Arc::clone(&blockchain), shutdown_tx);

    // Wait until shutdown signal is received
    shutdown_rx.await.unwrap();

    if let Ok(mut blockchain) = blockchain.lock() {
        blockchain.cleanup_if_developer_mode();
    }
    Ok(())
}

fn load_configuration(env: &str) -> Result<node::config::AppConfig, Box<dyn std::error::Error>> {
    let config = node::config::AppConfig::from_env(env)?;
    println!("Loaded configuration from env {:?}: {:?}", env, config);
    Ok(config)
}

fn initialize_blockchain(config: &node::config::AppConfig) -> Blockchain {
    Blockchain::new(
        config.blockchain_name.clone(),
        config.developer_mode.clone(),
        config.authorities.clone(),
    )
}