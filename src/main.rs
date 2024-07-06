use clap::Parser;
use std::sync::{Arc, Mutex};
use tokio::signal;
use tokio::sync::oneshot;
use node::blockchain::Blockchain;

mod node;
use node::websocket; 

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

    start_libp2p_service(config.clone(), shutdown_tx);
    start_websocket_service(config, Arc::clone(&blockchain));
    wait_for_shutdown_signal(shutdown_rx).await;

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

fn start_libp2p_service(config: node::config::AppConfig, shutdown_tx: oneshot::Sender<()>) {
    tokio::spawn(async move {
        if let Err(e) = node::libp2p::run(&config).await {
            eprintln!("Error running libp2p: {}", e);
        }
        let _ = shutdown_tx.send(());
    });
}

fn start_websocket_service(config: node::config::AppConfig, blockchain: Arc<Mutex<Blockchain>>) {
    let addr = config.websocket_addr.clone();

    tokio::spawn(async move {
        if let Err(e) = websocket::start_websocket_server(&addr, blockchain).await {
            eprintln!("Error starting WebSocket server: {}", e);
        }
    });
}

async fn wait_for_shutdown_signal(shutdown_rx: oneshot::Receiver<()>) {
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down.");
        }
        _ = shutdown_rx => {
            println!("Libp2p service completed, shutting down.");
        }
    }
}