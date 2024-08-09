use crate::node::{block::Block, p2p_server::GossipMessageType};
use crate::node::blockchain::Blockchain;
use crate::node::transaction::Transaction;
use crate::node::rlp_encoding::decode;

use libp2p::{
    gossipsub::{self, MessageId}, PeerId,
};
use std::sync::Arc;
use tokio::sync::Mutex;


pub async fn handle_gossipsub_message(
    peer_id: PeerId,
    id: MessageId,
    message: gossipsub::Message,
    blockchain: &Arc<Mutex<Blockchain>>,
) {
    println!(
        "Got message: '{}' with id: {id} from peer: {peer_id}",
        String::from_utf8_lossy(&message.data),
    );

    let message_type = GossipMessageType::from_byte(message.data[0]);
    let payload = &message.data[1..];

    match message_type {
        Some(GossipMessageType::Transaction) => match decode::<Transaction>(payload) {
            Ok(transaction) => {
                println!("Decoded transaction: {:?}", &transaction);
                handle_received_transaction(&transaction, blockchain).await;
            }
            Err(e) => {
                eprintln!("Failed to decode transaction: {:?}", e);
            }
        },
        Some(GossipMessageType::Block) => match decode::<Block>(payload) {
            Ok(block) => {
                println!("Decoded block: {:?}", &block);
                handle_received_block(&block, blockchain).await;
            }
            Err(e) => {
                eprintln!("Failed to decode block: {:?}", e);
            }
        },
        _ => {
            eprintln!("Unknown message type: {:?}", message_type);
        }
    }
}

async fn handle_received_transaction(
    transaction: &Transaction,
    blockchain: &Arc<Mutex<Blockchain>>,
) {
    let result = {
        let blockchain = blockchain.lock().await;
        blockchain.add_transaction_to_pool(&transaction)
    };

    match result {
        Ok(_) => println!("Transaction added to mempool from P2P"),
        Err(e) => println!("Failed to add transaction to pool: {:?}", e),
    }
}

async fn handle_received_block(block: &Block, blockchain: &Arc<Mutex<Blockchain>>) {
    let result = {
        let blockchain = blockchain.lock().await;
        blockchain.import_block(&block)
    };

    match result {
        Ok(_) => println!("Block added to blockchain from P2P"),
        Err(e) => println!("Failed to add block to blockchain: {:?}", e),
    }
}