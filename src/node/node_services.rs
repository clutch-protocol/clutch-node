use crate::node::blockchain::Blockchain;
use crate::node::config::AppConfig;
use crate::node::p2p_server::MessageType;
use crate::node::p2p_server::P2PServer;
use crate::node::p2p_server::P2PServerCommand;
use crate::node::rlp_encoding::encode;
use crate::node::websocket::WebSocket;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tokio::sync::{mpsc, oneshot, Mutex};

pub struct NodeServices;

impl NodeServices {
    pub async fn start_services(config: &AppConfig, blockchain: Blockchain) {
        let blockchain_arc = Arc::new(Mutex::new(blockchain));

        let (libp2p_shutdown_tx, libp2p_shutdown_rx) = oneshot::channel();
        let (command_tx_p2p, command_rx_p2p) = mpsc::channel(32);
        Self::start_libp2p(
            config,
            Arc::clone(&blockchain_arc),
            libp2p_shutdown_tx,
            command_rx_p2p,
        );

        let (websocket_shutdown_tx, websocket_shutdown_rx) = oneshot::channel();
        Self::start_websocket(
            config,
            Arc::clone(&blockchain_arc),
            command_tx_p2p.clone(),
            websocket_shutdown_tx,
        );

        Self::start_authoring_job(Arc::clone(&blockchain_arc), 1, command_tx_p2p.clone());

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
        let listen_addrs: Vec<&str> = config.listen_addrs.iter().map(|s| s.as_str()).collect();
        let bootstrap_nodes: Vec<&str> = config.bootstrap_nodes.iter().map(|s| s.as_str()).collect();
        
        let mut p2p_server = match P2PServer::new(&config.libp2p_topic_name, &listen_addrs,&bootstrap_nodes) {
            Ok(server) => server,
            Err(e) => {
                eprintln!("Failed to create P2PServer: {}", e);
                return;
            }
        };

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
        command_tx_p2p: tokio::sync::mpsc::Sender<P2PServerCommand>,
        websocket_shutdown_tx: oneshot::Sender<()>,
    ) {
        let websocket_addr = config.websocket_addr.clone();

        tokio::spawn(async move {
            if let Err(e) = WebSocket::run(&websocket_addr, blockchain, command_tx_p2p).await {
                eprintln!("Error starting WebSocket server: {}", e);
            }
            let _ = websocket_shutdown_tx.send(());
        });
    }

    pub fn start_authoring_job(
        blockchain: Arc<Mutex<Blockchain>>,
        interval_secs: u64,
        command_tx_p2p: tokio::sync::mpsc::Sender<P2PServerCommand>,
    ) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
            loop {
                interval.tick().await;
                let blockchain = blockchain.lock().await;
                match blockchain.author_new_block() {
                    Ok(block) => {
                        let encoded_block = encode(&block);
                        P2PServer::gossip_message(
                            command_tx_p2p.clone(),
                            MessageType::Block,
                            &encoded_block,
                        )
                        .await;
                    }
                    Err(_e) => {
                        // eprintln!("Error authoring new block: {}", e);
                    }
                }
            }
        });
    }
}
