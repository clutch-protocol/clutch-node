use crate::node::block::Block;
use crate::node::blockchain::Blockchain;
use crate::node::rlp_encoding::decode;
use crate::node::transaction::Transaction;
use futures::stream::StreamExt;
use futures::FutureExt;
use libp2p::{
    gossipsub::{self, Event as GossipsubEvent, IdentTopic, MessageId},
    mdns::{self, Event as MdnsEvent},
    noise,
    request_response::{
        cbor::Behaviour as RequestResponseBehavior, Config as RequestResponseConfig,
        Event as RequestResponseEvent, Message as RequestResponseMessage, OutboundRequestId,
        ProtocolSupport as RequestResponseProtocolSupport,
    },
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, StreamProtocol,
};

use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::{
    io, select,
    sync::{oneshot, Mutex},
};
use tracing_subscriber::EnvFilter;

#[derive(NetworkBehaviour)]
pub struct P2PBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    pub request_response: RequestResponseBehavior<DirectMessageRequest, DirectMessageResponse>,
}

pub struct P2PServer {
    pub behaviour: Swarm<P2PBehaviour>,
    pub topic: IdentTopic,
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

#[allow(dead_code)]
pub enum P2PServerCommand {
    SendGossipMessage {
        message: Vec<u8>,
        response_tx: tokio::sync::oneshot::Sender<Result<MessageId, gossipsub::PublishError>>,
    },
    GetConnectedPeers {
        response_tx: tokio::sync::oneshot::Sender<HashSet<PeerId>>,
    },
    SendDirectMessage {
        peer_id: PeerId,
        message: DirectMessageRequest,
        response_tx: tokio::sync::oneshot::Sender<OutboundRequestId>,
    },
    GetLocalPeerId {
        response_tx: tokio::sync::oneshot::Sender<PeerId>,
    },
}

#[derive(Debug)]
pub enum GossipMessageType {
    Transaction,
    Block,
}

impl GossipMessageType {
    fn as_byte(&self) -> u8 {
        match self {
            GossipMessageType::Transaction => 0x01,
            GossipMessageType::Block => 0x02,
        }
    }

    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(GossipMessageType::Transaction),
            0x02 => Some(GossipMessageType::Block),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DirectMessageRequest {
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DirectMessageResponse {
    pub message: String,
}
