use crate::node::blockchain::Blockchain;
use crate::node::transaction::Transaction;
use crate::node::p2p_server::{P2PServer, P2PServerCommand};
use crate::node::rlp_encoding::encode;
use futures::{stream::StreamExt, SinkExt};
use std::error::Error;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;


pub struct WebSocket;

impl WebSocket {
    pub async fn run(
        addr: &str,
        blockchain: Arc<Mutex<Blockchain>>,
        command_tx: tokio::sync::mpsc::Sender<P2PServerCommand>,
    ) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("WebSocket server started on {}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            let blockchain = Arc::clone(&blockchain);
            tokio::spawn(Self::handle_connection(stream, blockchain, command_tx.clone()));
        }

        Ok(())
    }

    async fn handle_connection(
        stream: TcpStream,
        blockchain: Arc<Mutex<Blockchain>>,
        command_tx: tokio::sync::mpsc::Sender<P2PServerCommand>,
    ) {
        match accept_async(stream).await {
            Ok(ws_stream) => {
                let (mut ws_sender, mut ws_receiver) = ws_stream.split();

                while let Some(Ok(message)) = ws_receiver.next().await {
                    if let Message::Text(text) = message {
                        println!("Received from websocket: {}", text);
                        let response = Self::handle_json_rpc_request(&text, &blockchain, command_tx.clone()).await;

                        if let Some(response) = response {
                            if let Err(e) = ws_sender.send(Message::Text(response)).await {
                                eprintln!("Error sending message: {}", e);
                                return;
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error during the websocket handshake: {}", e),
        }
    }

    async fn handle_json_rpc_request(
        request: &str,
        blockchain: &Arc<Mutex<Blockchain>>,
        command_tx: tokio::sync::mpsc::Sender<P2PServerCommand>,
    ) -> Option<String> {
        let request: serde_json::Value = match serde_json::from_str(request) {
            Ok(val) => val,
            Err(_) => return Some(serde_json::json!({"jsonrpc": "2.0", "error": {"code": -32700, "message": "Parse error"}, "id": null}).to_string()),
        };

        let method = request.get("method")?.as_str()?;
        let params = request.get("params")?;
        let id = request
            .get("id")
            .cloned()
            .unwrap_or(serde_json::json!(null));

        match method {
            "add_transaction" => {
                let transaction: Transaction = match serde_json::from_value(params.clone()) {
                    Ok(tx) => tx,
                    Err(_) => return Some(serde_json::json!({"jsonrpc": "2.0", "error": {"code": -32602, "message": "Invalid params"}, "id": id}).to_string()),
                };

                let  blockchain = blockchain.lock().await;
                if blockchain.add_transaction_to_pool(&transaction).is_ok() {
                    println!("Transaction added to pool from wss.");
                    
                    // gossip transcation                                        
                    let encoded_tx = encode(&transaction);
                    P2PServer::gossip_message(command_tx, &encoded_tx).await;
                    
                    return Some(serde_json::json!({"jsonrpc": "2.0", "result": "Transaction added", "id": id}).to_string());
                } else {
                    return Some(serde_json::json!({"jsonrpc": "2.0", "error": {"code": -32000, "message": "Failed to add transaction"}, "id": id}).to_string());
                }
            }
            _ => Some(serde_json::json!({"jsonrpc": "2.0", "error": {"code": -32601, "message": "Method not found"}, "id": id}).to_string()),
        }
    }
}
