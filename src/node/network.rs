use crate::node::blockchain::Blockchain;
use crate::node::config::AppConfig;
use crate::node::p2p_server::Command;
use crate::node::p2p_server::P2PServer;
use crate::node::websocket::WebSocket;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tokio::sync::{mpsc, oneshot, Mutex};
use rand::Rng; // Add this import for random string generation

pub struct Network;

impl Network {
    pub async fn start_services(config: &AppConfig, blockchain: Blockchain) {
        let blockchain_arc = Arc::new(Mutex::new(blockchain));
        let p2p_server_arc = Arc::new(Mutex::new(
            P2PServer::new(&config.libp2p_topic_name).unwrap(),
        ));

        let (libp2p_shutdown_tx, libp2p_shutdown_rx) = oneshot::channel();
        let (websocket_shutdown_tx, websocket_shutdown_rx) = oneshot::channel();

        Self::start_libp2p(
            Arc::clone(&blockchain_arc),
            Arc::clone(&p2p_server_arc),
            libp2p_shutdown_tx,
        );

        Self::start_websocket(
            config,
            Arc::clone(&blockchain_arc),
            Arc::clone(&p2p_server_arc),
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
        blockchain.cleanup_if_developer_mode();
    }

    fn start_libp2p(
        blockchain: Arc<Mutex<Blockchain>>,
        p2p_server: Arc<Mutex<P2PServer>>,
        libp2p_shutdown_tx: oneshot::Sender<()>,
    ) {
        let (command_tx, command_rx) = mpsc::channel(32);
        tokio::spawn(async move {
            {
                let mut p2p_server = p2p_server.lock().await;
                if let Err(e) = p2p_server.run(Arc::clone(&blockchain), command_rx).await {
                    eprintln!("Error running libp2p: {}", e);
                }
            }
            let _ = libp2p_shutdown_tx.send(());
        });

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(3));
            loop {
                interval.tick().await;
                
                // Generate a random string
                let random_string: String = rand::thread_rng()
                    .sample_iter(&rand::distributions::Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect();

                // Example of sending a message
                let (response_tx, response_rx) = oneshot::channel();
                command_tx
                    .send(Command::SendMessage {
                        message: random_string.clone(),
                        response_tx,
                    })
                    .await
                    .unwrap();

                match response_rx.await {
                    Ok(result) => match result {
                        Ok(message_id) => println!("Message sent with id: {:?}", message_id),
                        Err(e) => eprintln!("Failed to send message: {:?}", e),
                    },
                    Err(e) => eprintln!("Failed to receive response: {:?}", e),
                }
            }
        });
    }

    fn start_websocket(
        config: &AppConfig,
        blockchain: Arc<Mutex<Blockchain>>,
        p2p_server: Arc<Mutex<P2PServer>>,
        websocket_shutdown_tx: oneshot::Sender<()>,
    ) {
        let websocket_addr = config.websocket_addr.clone();

        tokio::spawn(async move {
            if let Err(e) = WebSocket::run(&websocket_addr, blockchain, p2p_server).await {
                eprintln!("Error starting WebSocket server: {}", e);
            }
            let _ = websocket_shutdown_tx.send(());
        });
    }
}
