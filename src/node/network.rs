use crate::node::blockchain::Blockchain;
use crate::node::config::AppConfig;
use crate::node::p2p_server::P2PServer;
use crate::node::p2p_server::P2PServerCommand;
use crate::node::websocket::WebSocket;
use std::sync::Arc;
use tokio::signal;
use tokio::sync::{mpsc, oneshot, Mutex};

pub struct Network;

impl Network {
    pub async fn start_services(config: &AppConfig, blockchain: Blockchain) {
        let blockchain_arc = Arc::new(Mutex::new(blockchain));

        let (libp2p_shutdown_tx, libp2p_shutdown_rx) = oneshot::channel();
        let (command_tx, command_rx) = mpsc::channel(32);
        Self::start_libp2p(
            config,
            Arc::clone(&blockchain_arc),
            libp2p_shutdown_tx,
            command_rx,
        );

        let (websocket_shutdown_tx, websocket_shutdown_rx) = oneshot::channel();
        Self::start_websocket(
            config,
            Arc::clone(&blockchain_arc),
            command_tx,
            websocket_shutdown_tx,
        );

        Self::wait_for_shutdown_signal(
            libp2p_shutdown_rx,
            websocket_shutdown_rx,
            Arc::clone(&blockchain_arc),
        )
        .await;
    }

    async fn wait_for_shutdown_signal(
        libp2p_shutdown_rx: oneshot::Receiver<()>,
        websocket_shutdown_rx: oneshot::Receiver<()>,
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

        let mut blockchain = blockchain.lock().await;
        blockchain.shutdown_blockchain();
    }

    fn start_libp2p(
        config: &AppConfig,
        blockchain: Arc<Mutex<Blockchain>>,
        libp2p_shutdown_tx: oneshot::Sender<()>,
        command_rx: tokio::sync::mpsc::Receiver<P2PServerCommand>,
    ) {
        let mut p2p_server = P2PServer::new(&config.libp2p_topic_name).unwrap();
        tokio::spawn(async move {
            {
                if let Err(e) = p2p_server.run(Arc::clone(&blockchain), command_rx).await {
                    eprintln!("Error running libp2p: {}", e);
                }
            }
            let _ = libp2p_shutdown_tx.send(());
        });
    }

    fn start_websocket(
        config: &AppConfig,
        blockchain: Arc<Mutex<Blockchain>>,
        command_tx: tokio::sync::mpsc::Sender<P2PServerCommand>,
        websocket_shutdown_tx: oneshot::Sender<()>,
    ) {
        let websocket_addr = config.websocket_addr.clone();

        tokio::spawn(async move {
            if let Err(e) = WebSocket::run(&websocket_addr, blockchain, command_tx).await {
                eprintln!("Error starting WebSocket server: {}", e);
            }
            let _ = websocket_shutdown_tx.send(());
        });
    }
}
