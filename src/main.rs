use clap::Parser;

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
    let blockchain=  initialize_blockchain(&config);
    
    blockchain.start_network_services(&config).await;
    Ok(())
}

fn load_configuration(env: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config = AppConfig::from_env(env)?;
    println!("Loaded configuration from env {:?}: {:?}", env, config);
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
