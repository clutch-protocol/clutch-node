use std::collections::HashSet;
use libp2p::{gossipsub::{self, MessageId}, request_response::OutboundRequestId, PeerId};
use tokio::sync::oneshot;

use super::behaviour::DirectMessageRequest;

#[allow(dead_code)]
pub enum P2PServerCommand {
    SendGossipMessage {
        message: Vec<u8>,
        response_tx: oneshot::Sender<Result<MessageId, gossipsub::PublishError>>,
    },
    GetConnectedPeers {
        response_tx: oneshot::Sender<HashSet<PeerId>>,
    },
    SendDirectMessage {
        peer_id: PeerId,
        message: DirectMessageRequest,
        response_tx: oneshot::Sender<OutboundRequestId>,
    },
    GetLocalPeerId {
        response_tx: oneshot::Sender<PeerId>,
    },
}

#[derive(Debug)]
pub enum GossipMessageType {
    Transaction,
    Block,
}

impl GossipMessageType {
    pub fn as_byte(&self) -> u8 {
        match self {
            GossipMessageType::Transaction => 0x01,
            GossipMessageType::Block => 0x02,
        }
    }

    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(GossipMessageType::Transaction),
            0x02 => Some(GossipMessageType::Block),
            _ => None,
        }
    }
}
