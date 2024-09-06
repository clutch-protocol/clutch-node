use clap::Parser;
mod node;
use node::blockchain::Blockchain;
use node::configuration::AppConfig;
use node::tracing::setup_tracing;

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
    let config = AppConfig::load_configuration(env)?;
    setup_tracing(&config.log_level, &config.seq_url, &config.seq_api_key)?;

    let blockchain = initialize_blockchain(&config);
    blockchain.start_network_services(&config).await;
    Ok(())
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
