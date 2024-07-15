use crate::node::blockchain::Blockchain;
use crate::node::config::AppConfig;
use crate::node::libp2p::P2PServer;
use crate::node::websocket::WebSocket;
use std::sync::{Arc, Mutex};
use tokio::signal;
use tokio::sync::oneshot;

pub struct Network;

impl Network {
    pub async fn start_services(
        config: &AppConfig,
        blockchain: Blockchain,
        shutdown_tx: oneshot::Sender<()>,
    ) {
        let blockchain_arc = Arc::new(Mutex::new(blockchain));

        let (libp2p_shutdown_tx, libp2p_shutdown_rx) = oneshot::channel();
        let (websocket_shutdown_tx, websocket_shutdown_rx) = oneshot::channel();

        // Start libp2p service
        let libp2p_config = config.clone();
        let libp2p_blockchain = Arc::clone(&blockchain_arc);
        tokio::spawn(async move {
            let topic_name = &libp2p_config.libp2p_topic_name;
            let mut p2p_server = P2PServer::new(&topic_name);
            if let Err(e) = p2p_server.run(libp2p_blockchain).await {
                eprintln!("Error running libp2p: {}", e);
            }
            let _ = libp2p_shutdown_tx.send(());
        });

        // Start WebSocket service
        let websocket_config = config.clone();
        let websocket_blockchain = Arc::clone(&blockchain_arc);
        tokio::spawn(async move {
            let addr = &websocket_config.websocket_addr;
            if let Err(e) = WebSocket::run(addr, websocket_blockchain).await {
                eprintln!("Error starting WebSocket server: {}", e);
            }
            let _ = websocket_shutdown_tx.send(());
        });

        // Wait for shutdown signal
        Self::wait_for_shutdown_signal(
            libp2p_shutdown_rx,
            websocket_shutdown_rx,
            shutdown_tx,
            blockchain_arc,
        )
        .await;
    }

    async fn wait_for_shutdown_signal(
        libp2p_shutdown_rx: oneshot::Receiver<()>,
        websocket_shutdown_rx: oneshot::Receiver<()>,
        shutdown_tx: oneshot::Sender<()>,
        blockchain: Arc<Mutex<Blockchain>>,
    ) {
        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("Received Ctrl+C, shutting down.");
            }
            _ = libp2p_shutdown_rx => {
                println!("Libp2p service completed, shutting down.");
            }
            _ = websocket_shutdown_rx => {
                println!("WebSocket service completed, shutting down.");
            }
        }

        // Send shutdown signal
        let _ = shutdown_tx.send(());

        // Cleanup blockchain if in developer mode
        if let Ok(mut blockchain) = blockchain.lock() {
            blockchain.cleanup_if_developer_mode();
        }
    }
}
