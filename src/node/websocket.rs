use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures::stream::StreamExt;
use futures::SinkExt;
use crate::node::blockchain::Blockchain;

pub async fn start_websocket_server(addr: &str, blockchain: Arc<Mutex<Blockchain>>) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    println!("WebSocket server started on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let blockchain = Arc::clone(&blockchain);
        tokio::spawn(handle_websocket_connection(stream, blockchain));
    }

    Ok(())
}

async fn handle_websocket_connection(stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
    let ws_stream = accept_async(stream).await.expect("Error during the websocket handshake");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(Ok(message)) = ws_receiver.next().await {
        if let Message::Text(text) = message {
            println!("Received: {}", text);
            
            if let Ok(mut blockchain) = blockchain.lock() {
                // blockchain.add_transaction_to_pool(transaction);
            }

            if let Err(e) = ws_sender.send(Message::Text(text)).await {
                eprintln!("Error sending message: {}", e);
                return;
            }
        }
    }
}
