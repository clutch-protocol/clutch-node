use crate::node::blockchain::Blockchain;
use crate::node::config::AppConfig;
use crate::node::libp2p;
use crate::node::websocket;
use std::sync::{Arc, Mutex};
use tokio::signal;
use tokio::sync::oneshot;

pub struct Network;

impl Network {
    pub fn start_services(
        config: &AppConfig,
        blockchain: Arc<Mutex<Blockchain>>,
        shutdown_tx: oneshot::Sender<()>,
    ) {
        let (libp2p_shutdown_tx, libp2p_shutdown_rx) = oneshot::channel();
        let (websocket_shutdown_tx, websocket_shutdown_rx) = oneshot::channel();

        Self::start_libp2p_service(config, Arc::clone(&blockchain), libp2p_shutdown_tx);
        Self::start_websocket_service(config, Arc::clone(&blockchain), websocket_shutdown_tx);

        tokio::spawn(async move {
            Network::wait_for_shutdown_signal(libp2p_shutdown_rx, websocket_shutdown_rx, shutdown_tx, blockchain).await;
        });
    }

    fn start_libp2p_service(
        config: &AppConfig,
        blockchain: Arc<Mutex<Blockchain>>,
        shutdown_tx: oneshot::Sender<()>,
    ) {
        let config = config.clone();
        tokio::spawn(async move {
            if let Err(e) = libp2p::run(&config, blockchain).await {
                eprintln!("Error running libp2p: {}", e);
            }
            let _ = shutdown_tx.send(());
        });
    }

    fn start_websocket_service(config: &AppConfig, blockchain: Arc<Mutex<Blockchain>>, shutdown_tx: oneshot::Sender<()>) {
        let addr = config.websocket_addr.clone();
        tokio::spawn(async move {
            if let Err(e) = websocket::start_websocket_server(&addr, blockchain).await {
                eprintln!("Error starting WebSocket server: {}", e);
            }
            let _ = shutdown_tx.send(());
        });
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
        let _ = shutdown_tx.send(());

        if let Ok(mut blockchain) = blockchain.lock() {
            blockchain.cleanup_if_developer_mode();
        }
    }
}
