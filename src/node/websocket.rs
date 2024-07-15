use crate::node::network::Network;
use crate::node::transaction::Transaction;
use futures::stream::StreamExt;
use futures::SinkExt;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

pub struct WebSocket;

impl WebSocket {

    pub async fn run(
        addr: &str,
        blockchain: Arc<Mutex<Network>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("WebSocket server started on {}", addr);
    
        while let Ok((stream, _)) = listener.accept().await {
            let blockchain = Arc::clone(&blockchain);
            tokio::spawn(Self::handle_websocket_connection(stream, blockchain));
        }
    
        Ok(())
    }
    
    async fn handle_websocket_connection(stream: TcpStream, blockchain: Arc<Mutex<Network>>) {
        let ws_stream = accept_async(stream)
            .await
            .expect("Error during the websocket handshake");
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    
        while let Some(Ok(message)) = ws_receiver.next().await {
            if let Message::Text(text) = message {
                println!("Received from wss: {}", text);
    
                let response = Self::handle_json_rpc_request(&text, &blockchain).await;
    
                if let Some(response) = response {
                    if let Err(e) = ws_sender.send(Message::Text(response)).await {
                        eprintln!("Error sending message: {}", e);
                        return;
                    }
                }
            }
        }
    }
    
    async fn handle_json_rpc_request(
        request: &str,
        network: &Arc<Mutex<Network>>,
    ) -> Option<String> {
        let request: Value = match serde_json::from_str(request) {
            Ok(val) => val,
            Err(_) => return Some(json!({"jsonrpc": "2.0", "error": {"code": -32700, "message": "Parse error"}, "id": null}).to_string()),
        };
    
        let method = request.get("method")?.as_str()?;
        let params = request.get("params")?;
        let id = request.get("id").cloned().unwrap_or(json!(null));
    
        match method {
            "add_transaction" => {
                let transaction: Transaction = match serde_json::from_value(params.clone()) {
                    Ok(tx) => tx,
                    Err(_) => return Some(json!({"jsonrpc": "2.0", "error": {"code": -32602, "message": "Invalid params"}, "id": id}).to_string()),
                };
    
                let response = match network.lock() {
                    Ok(net) => {
                        if net.blockchain.lock().unwrap().add_transaction_to_pool(transaction).is_ok() {
                            // net.libp2p.unwrap().lock().unwrap().broadcast_transaction(transaction);
                            json!({"jsonrpc": "2.0", "result": "Transaction added", "id": id}).to_string()
                        } else {
                            json!({"jsonrpc": "2.0", "error": {"code": -32000, "message": "Failed to add transaction"}, "id": id}).to_string()
                        }
                    }
                    Err(_) => json!({"jsonrpc": "2.0", "error": {"code": -32000, "message": "Failed to lock blockchain"}, "id": id}).to_string(),
                };

                Some(response)
            }
            _ => Some(json!({"jsonrpc": "2.0", "error": {"code": -32601, "message": "Method not found"}, "id": id}).to_string()),
        }
    }
    
}
