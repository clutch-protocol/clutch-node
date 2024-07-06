use clap::Parser;
use node::blockchain::Blockchain;
use tokio::signal;
use tokio::sync::oneshot;

mod node;

/// Command line arguments structure
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
    let mut blockchain = initialize_blockchain(&config);
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    start_libp2p_task(config, shutdown_tx).await;
    await_shutdown_signal(shutdown_rx).await;
    blockchain.cleanup_if_developer_mode();
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

async fn start_libp2p_task(config: node::config::AppConfig, shutdown_tx: oneshot::Sender<()>) {
    tokio::spawn(async move {
        if let Err(e) = node::libp2p::run(config).await {
            eprintln!("Error running libp2p: {}", e);
        }
        let _ = shutdown_tx.send(());
    });
}

async fn await_shutdown_signal(shutdown_rx: oneshot::Receiver<()>) {
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down.");
        }
        _ = shutdown_rx => {
            println!("Libp2p task completed, shutting down.");
        }
    }
}
