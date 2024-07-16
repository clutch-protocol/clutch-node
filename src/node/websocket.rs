use crate::node::blockchain::Blockchain;
use crate::node::libp2p::P2PServer;
use crate::node::transaction::Transaction;
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
        p2p_server: Arc<Mutex<P2PServer>>,
    ) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("WebSocket server started on {}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            let blockchain = Arc::clone(&blockchain);
            let p2p_server = Arc::clone(&p2p_server);
            tokio::spawn(Self::handle_connection(stream, blockchain, p2p_server));
        }

        Ok(())
    }

    async fn handle_connection(
        stream: TcpStream,
        blockchain: Arc<Mutex<Blockchain>>,
        p2p_server: Arc<Mutex<P2PServer>>,
    ) {
        let ws_stream = accept_async(stream)
            .await
            .expect("Error during the websocket handshake");
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        while let Some(Ok(message)) = ws_receiver.next().await {
            if let Message::Text(text) = message {
                println!("Received from websocket: {}", text);
                let response = Self::handle_json_rpc_request(&text, &blockchain, &p2p_server).await;

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
        blockchain: &Arc<Mutex<Blockchain>>,
        p2p_server: &Arc<Mutex<P2PServer>>,
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
                if blockchain.add_transaction_to_pool(&transaction).is_ok(){

                    p2p_server.lock().await.broadcast_transaction(&transaction);

                    return  Some(serde_json::json!({"jsonrpc": "2.0", "result": "Transaction added", "id": id}).to_string());
                }
                else {
                    return  Some(serde_json::json!({"jsonrpc": "2.0", "error": {"code": -32000, "message": "Failed to add transaction"}, "id": id}).to_string());
                }              
            }
            _ => Some(serde_json::json!({"jsonrpc": "2.0", "error": {"code": -32601, "message": "Method not found"}, "id": id}).to_string()),
        }
    }
}
