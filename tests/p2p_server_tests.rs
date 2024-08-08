use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use clutch_node::node::blockchain::Blockchain;
use clutch_node::node::p2p_server::{P2PServer, P2PServerCommand, MessageType};
use std::time::Duration;

async fn setup_p2p_server(topic_name: &str, listen_addrs: &[&str], peer_addrs: &[&str], blockchain: Arc<Mutex<Blockchain>>) -> (Arc<Mutex<P2PServer>>, tokio::sync::mpsc::Sender<P2PServerCommand>) {
    let server: Arc<Mutex<P2PServer>> = Arc::new(Mutex::new(P2PServer::new(
        topic_name,
        listen_addrs,
        peer_addrs,
    ).unwrap()));

    let (command_tx, command_rx) = mpsc::channel(32);

    let blockchain_clone = Arc::clone(&blockchain);
    let server_clone = Arc::clone(&server);

    tokio::spawn(async move {
        server_clone.lock().await.run(blockchain_clone, command_rx).await.unwrap();
    });

    tokio::time::sleep(Duration::from_secs(1)).await;
    (server, command_tx)
}

fn initialize_blockchain(name: String) -> Blockchain {
    Blockchain::new(
        name,
        "0x9b6e8afff8329743cac73dbef83ca3cbf9a74c20".to_string(),
        "0883ddd3d07303b87c954b0c9383f7b78f45e002520fc03a8adc80595dbf6509".to_string(),
        true,
        vec!["0x9b6e8afff8329743cac73dbef83ca3cbf9a74c20".to_string()],
    )
}

#[tokio::test]
async fn test_p2p_server_gossip_message() {
    let topic_name = "test-topic";

    // Initialize blockchain
    let blockchain = Arc::new(Mutex::new(initialize_blockchain("clutch-node-test-1".to_string())));

    // Setup servers
    let (_server1, command_tx1) = setup_p2p_server(topic_name, &["/ip4/127.0.0.1/tcp/4001"], &["/ip4/127.0.0.1/tcp/4002"], Arc::clone(&blockchain)).await;
    let (_server2, command_tx2) = setup_p2p_server(topic_name, &["/ip4/127.0.0.1/tcp/4002"], &["/ip4/127.0.0.1/tcp/4001"], Arc::clone(&blockchain)).await;

    // Send a message from server1 to server2
    let message = b"Hello, world!".to_vec();
    P2PServer::gossip_message(command_tx1.clone(), MessageType::Transaction, &message).await;

    // Wait for the message to propagate
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Check if the message was received by server2
    // This part depends on how you want to validate the message reception.
    // For simplicity, we're printing the message in the handle_gossipsub_message method.
    // You can add a flag or counter to verify it here.

    // Shut down the servers
    drop(command_tx1);
    drop(command_tx2);
    blockchain.lock().await.shutdown_blockchain();
}

#[tokio::test]
async fn test_p2p_server_connected_peers() {
    let topic_name = "test-topic";

    // Initialize blockchain
    let blockchain = Arc::new(Mutex::new(initialize_blockchain("clutch-node-test-1".to_string())));

    // Setup servers
    let (_server1, command_tx1) = setup_p2p_server(topic_name, &["/ip4/127.0.0.1/tcp/4001"], &["/ip4/127.0.0.1/tcp/4002"], Arc::clone(&blockchain)).await;
    let (_server2, command_tx2) = setup_p2p_server(topic_name, &["/ip4/127.0.0.1/tcp/4002"], &["/ip4/127.0.0.1/tcp/4001"], Arc::clone(&blockchain)).await;

    // Wait for the peers to connect
    tokio::time::sleep(Duration::from_secs(1)).await;

    // // Check connected peers
    let connected_peers_server1 = P2PServer::get_connected_peers_command(command_tx1.clone()).await.unwrap();
    let connected_peers_server2 = P2PServer::get_connected_peers_command(command_tx2.clone()).await.unwrap();

    println!("Server 1 connected peers: {:?}", connected_peers_server1);
    println!("Server 2 connected peers: {:?}", connected_peers_server2);

    // Shut down the servers
    drop(command_tx1);
    drop(command_tx2);
    blockchain.lock().await.shutdown_blockchain();
}
