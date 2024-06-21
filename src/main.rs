use clap::Parser;
use node::blockchain::Blockchain;
use tokio::signal;
use tokio::sync::oneshot;

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
    let config = node::config::AppConfig::from_env(&env)?;
    println!("Loaded configuration from env {:?}: {:?}", &env, &config);

    let mut blockchain = Blockchain::new(
        config.blockchain_name.clone(),
        config.developer_mode.clone(),
        config.authorities.clone(),
    );

    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    tokio::spawn(async move {
        if let Err(e) = node::libp2p::run(config).await {
            eprintln!("Error running libp2p: {}", e);
        }
        let _ = shutdown_tx.send(());
    });

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down.");
        }
        _ = shutdown_rx => {
            println!("Libp2p task completed, shutting down.");
        }
    }

    blockchain.cleanup_if_developer_mode();
    Ok(())
}
