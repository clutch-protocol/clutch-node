use crate::node::blockchain::Blockchain;
use crate::node::libp2p::P2PBehaviour;
use crate::node::websocket::WebSocket;
use crate::node::config::AppConfig;
use std::sync::{Arc, Mutex};
use tokio::signal;
use tokio::sync::oneshot;

pub struct Network {
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub libp2p: Option<Arc<Mutex<P2PBehaviour>>>,
    pub websocket: Option<Arc<Mutex<WebSocket>>>,
}

impl Network {
    pub fn new(blockchain: Blockchain) -> Self {
        Network {
            blockchain: Arc::new(Mutex::new(blockchain)),
            libp2p: None,
            websocket: None,
        }
    }

    pub async fn start_services(
        config: &AppConfig,
        blockchain: Blockchain,
        shutdown_tx: oneshot::Sender<()>,
    ) {
        let network = Arc::new(Mutex::new(Network::new(blockchain)));

        let (libp2p_shutdown_tx, libp2p_shutdown_rx) = oneshot::channel();
        let (websocket_shutdown_tx, websocket_shutdown_rx) = oneshot::channel();

        // Start libp2p service
        let libp2p_config = config.clone();
        let libp2p_network = Arc::clone(&network);
        tokio::spawn(async move {
            let topic_name = &libp2p_config.libp2p_topic_name;
            if let Err(e) = P2PBehaviour::run(topic_name, libp2p_network).await {
                eprintln!("Error running libp2p: {}", e);
            }
            let _ = libp2p_shutdown_tx.send(());
        });

        // Start WebSocket service
        let websocket_config = config.clone();
        let websocket_network = Arc::clone(&network);
        tokio::spawn(async move {
            let addr = &websocket_config.websocket_addr;
            if let Err(e) = WebSocket::run(addr, websocket_network).await {
                eprintln!("Error starting WebSocket server: {}", e);
            }
            let _ = websocket_shutdown_tx.send(());
        });

        // Wait for shutdown signal
        Self::wait_for_shutdown_signal(
            libp2p_shutdown_rx,
            websocket_shutdown_rx,
            shutdown_tx,
            network,
        )
        .await;
    }

    async fn wait_for_shutdown_signal(
        libp2p_shutdown_rx: oneshot::Receiver<()>,
        websocket_shutdown_rx: oneshot::Receiver<()>,
        shutdown_tx: oneshot::Sender<()>,
        network: Arc<Mutex<Network>>,
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
        if let Ok(net) = network.lock() {
            if let Ok(mut blockchain) = net.blockchain.lock() {
                blockchain.cleanup_if_developer_mode();
            }
        }
    }
}
