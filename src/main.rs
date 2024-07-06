use clap::Parser;
use futures::stream::StreamExt;
use futures::SinkExt;
use node::blockchain::Blockchain;
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
use tokio::sync::oneshot;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

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
    start_libp2p_task(config.clone(), shutdown_tx).await;

    start_websocket_server_task(&config);
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
        if let Err(e) = node::libp2p::run(&config).await {
            eprintln!("Error running libp2p: {}", e);
        }
        let _ = shutdown_tx.send(());
    });
}

fn start_websocket_server_task(config: &node::config::AppConfig) {
    let addr = config.websocket_addr.clone();

    tokio::spawn(async move {
        if let Err(e) = start_websocket_server(&addr).await {
            eprintln!("Error starting WebSocket server: {}", e);
        }
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

async fn start_websocket_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    println!("WebSocket server started on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_websocket_connection(stream));
    }

    Ok(())
}

async fn handle_websocket_connection(stream: TcpStream) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during the websocket handshake");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(Ok(message)) = ws_receiver.next().await {
        if let Message::Text(text) = message {
            println!("Received: {}", text);

            if let Err(e) = ws_sender.send(Message::Text(text)).await {
                eprintln!("Error sending message: {}", e);
                return;
            }
        }
    }
}
