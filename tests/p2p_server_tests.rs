use clutch_node::node::blockchain::Blockchain;
use clutch_node::node::get_block_header::GetBlockHeaders;
use clutch_node::node::handshake::Handshake;
use clutch_node::node::p2p_server::commands::DirectMessageType;
use clutch_node::node::p2p_server::{GossipMessageType, P2PServer, P2PServerCommand};
use clutch_node::node::rlp_encoding::encode;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

async fn setup_p2p_server(
    topic_name: &str,
    listen_addrs: &[&str],
    peer_addrs: &[&str],
    blockchain: Arc<Mutex<Blockchain>>,
) -> (
    Arc<Mutex<P2PServer>>,
    tokio::sync::mpsc::Sender<P2PServerCommand>,
) {
    let server: Arc<Mutex<P2PServer>> = Arc::new(Mutex::new(
        P2PServer::new(topic_name, listen_addrs, peer_addrs).unwrap(),
    ));

    let (command_tx, command_rx) = mpsc::channel(32);

    let blockchain_clone = Arc::clone(&blockchain);
    let server_clone = Arc::clone(&server);

    tokio::spawn(async move {
        server_clone
            .lock()
            .await
            .run(blockchain_clone, command_rx)
            .await
            .unwrap();
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
    let blockchain = Arc::new(Mutex::new(initialize_blockchain(
        "clutch-node-test-1".to_string(),
    )));

    // Setup servers
    let (_server1, command_tx1) = setup_p2p_server(
        topic_name,
        &["/ip4/127.0.0.1/tcp/4001"],
        &["/ip4/127.0.0.1/tcp/4002"],
        Arc::clone(&blockchain),
    )
    .await;
    let (_server2, command_tx2) = setup_p2p_server(
        topic_name,
        &["/ip4/127.0.0.1/tcp/4002"],
        &["/ip4/127.0.0.1/tcp/4001"],
        Arc::clone(&blockchain),
    )
    .await;

    // Send a message from server1 to server2
    let message = b"Hello, world!".to_vec();
    P2PServer::gossip_message_command(
        command_tx1.clone(),
        GossipMessageType::Transaction,
        &message,
    )
    .await;

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
    let blockchain = Arc::new(Mutex::new(initialize_blockchain(
        "clutch-node-test-1".to_string(),
    )));

    // Setup servers
    let (_server1, command_tx1) = setup_p2p_server(
        topic_name,
        &["/ip4/127.0.0.1/tcp/4001"],
        &["/ip4/127.0.0.1/tcp/4003"],
        Arc::clone(&blockchain),
    )
    .await;
    let (_server2, command_tx2) = setup_p2p_server(
        topic_name,
        &["/ip4/127.0.0.1/tcp/4002"],
        &["/ip4/127.0.0.1/tcp/4001"],
        Arc::clone(&blockchain),
    )
    .await;

    let (_server3, command_tx3) = setup_p2p_server(
        topic_name,
        &["/ip4/127.0.0.1/tcp/4003"],
        &["/ip4/127.0.0.1/tcp/4002"],
        Arc::clone(&blockchain),
    )
    .await;

    // Wait for the peers to connect
    tokio::time::sleep(Duration::from_secs(1)).await;

    let peer_id_server1 = P2PServer::get_local_peer_id_command(command_tx1.clone()).await;
    let peer_id_server2 = P2PServer::get_local_peer_id_command(command_tx2.clone()).await;
    let peer_id_server3 = P2PServer::get_local_peer_id_command(command_tx3.clone()).await;

    // // Check connected peers
    let connected_peers_server1 = P2PServer::get_connected_peers_command(command_tx1.clone())
        .await
        .unwrap();

    let connected_peers_server2 = P2PServer::get_connected_peers_command(command_tx2.clone())
        .await
        .unwrap();

    let connected_peers_server3 = P2PServer::get_connected_peers_command(command_tx3.clone())
        .await
        .unwrap();

    println!(
        "peer_id server 1: {:?}, connected peers: {:?}",
        peer_id_server1, connected_peers_server1
    );

    println!(
        "peer_id server 2: {:?}, connected peers: {:?}",
        peer_id_server2, connected_peers_server2
    );

    println!(
        "peer_id server 3: {:?}, connected peers: {:?}",
        peer_id_server3, connected_peers_server3
    );

    // Ensure peers are connected
    let connected_peers = P2PServer::get_connected_peers_command(command_tx1.clone())
        .await
        .unwrap();

    assert!(connected_peers.contains(&peer_id_server2));

    // Shut down the servers
    drop(command_tx1);
    drop(command_tx2);
    blockchain.lock().await.shutdown_blockchain();
}

#[tokio::test]
async fn test_p2p_server_get_local_peer_id() {
    let topic_name = "test-topic";

    // Initialize blockchain
    let blockchain = Arc::new(Mutex::new(initialize_blockchain(
        "clutch-node-test-1".to_string(),
    )));

    // Setup servers
    let (_server1, command_tx1) = setup_p2p_server(
        topic_name,
        &["/ip4/127.0.0.1/tcp/4001"],
        &["/ip4/127.0.0.1/tcp/4002"],
        Arc::clone(&blockchain),
    )
    .await;
    let (_server2, command_tx2) = setup_p2p_server(
        topic_name,
        &["/ip4/127.0.0.1/tcp/4002"],
        &["/ip4/127.0.0.1/tcp/4001"],
        Arc::clone(&blockchain),
    )
    .await;

    // Wait for the peers to connect
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Send a direct message from server1 to server2
    let peer_id = P2PServer::get_local_peer_id_command(command_tx1.clone()).await;
    println!("peer_id server 1: {:?}", peer_id);

    let peer_id = P2PServer::get_local_peer_id_command(command_tx2.clone()).await;
    println!("peer_id server 2: {:?}", peer_id);

    // Shut down the servers
    drop(command_tx1);
    drop(command_tx2);
    blockchain.lock().await.shutdown_blockchain();
}

#[tokio::test]
async fn test_p2p_server_handshake_direct_message() {
    let topic_name = "test-topic";

    // Initialize blockchain
    let blockchain = Arc::new(Mutex::new(initialize_blockchain(
        "clutch-node-test-1".to_string(),
    )));

    // Setup servers
    let (_server1, command_tx1) = setup_p2p_server(
        topic_name,
        &["/ip4/127.0.0.1/tcp/4001"],
        &["/ip4/127.0.0.1/tcp/4002"],
        Arc::clone(&blockchain),
    )
    .await;
    let (_server2, command_tx2) = setup_p2p_server(
        topic_name,
        &["/ip4/127.0.0.1/tcp/4002"],
        &["/ip4/127.0.0.1/tcp/4001"],
        Arc::clone(&blockchain),
    )
    .await;

    // Wait for the peers to connect
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Get peer IDs
    let peer_id_server1 = P2PServer::get_local_peer_id_command(command_tx1.clone()).await;
    let peer_id_server2 = P2PServer::get_local_peer_id_command(command_tx2.clone()).await;
    println!("peer_id server 1: {:?}", peer_id_server1);
    println!("peer_id server 2: {:?}", peer_id_server2);

    let handshake = Handshake {
        genesis_block_hash: "0xgenesis".to_string(),
        latest_block_hash: "0xlatest".to_string(),
        latest_block_index: 0,
    };

    let encoded_handshake = encode(&handshake);

    // Send a direct message from server2 to server1
    let request_id = P2PServer::send_direct_message_command(
        command_tx2.clone(),
        peer_id_server1,
        DirectMessageType::Handshake,
        &encoded_handshake,
    )
    .await
    .unwrap();

    println!("Request ID: {:?}", request_id);

    // Wait for the response or the event that handles the message
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Shut down the servers
    drop(command_tx1);
    drop(command_tx2);
    blockchain.lock().await.shutdown_blockchain();
}

#[tokio::test]
async fn test_p2p_server_get_block_headers_direct_message() {
    let topic_name = "test-topic";

    // Initialize blockchain
    let blockchain = Arc::new(Mutex::new(initialize_blockchain(
        "clutch-node-test-1".to_string(),
    )));

    // Setup servers
    let (_server1, command_tx1) = setup_p2p_server(
        topic_name,
        &["/ip4/127.0.0.1/tcp/4001"],
        &["/ip4/127.0.0.1/tcp/4002"],
        Arc::clone(&blockchain),
    )
    .await;
    let (_server2, command_tx2) = setup_p2p_server(
        topic_name,
        &["/ip4/127.0.0.1/tcp/4002"],
        &["/ip4/127.0.0.1/tcp/4001"],
        Arc::clone(&blockchain),
    )
    .await;

    // Wait for the peers to connect
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Get peer IDs
    let peer_id_server1 = P2PServer::get_local_peer_id_command(command_tx1.clone()).await;
    let peer_id_server2 = P2PServer::get_local_peer_id_command(command_tx2.clone()).await;
    println!("peer_id server 1: {:?}", peer_id_server1);
    println!("peer_id server 2: {:?}", peer_id_server2);

    let get_block_headers = GetBlockHeaders {
        start_block_hash: "2282c46637becfbe1f0f11d6d7d878ba1fa6c41fe5cad3bbdede42f6e5ac36e3".to_string(),
        skip: 0,
        limit: 100,
    };

    let encoded_get_block_headers = encode(&get_block_headers);

    // Send a direct message from server2 to server1
    let request_id = P2PServer::send_direct_message_command(
        command_tx2.clone(),
        peer_id_server1,
        DirectMessageType::GetBlockHeaders,
        &encoded_get_block_headers,
    )
    .await
    .unwrap();

    println!("Request ID: {:?}", request_id);

    // Wait for the response or the event that handles the message
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Shut down the servers
    drop(command_tx1);
    drop(command_tx2);
    blockchain.lock().await.shutdown_blockchain();
}
