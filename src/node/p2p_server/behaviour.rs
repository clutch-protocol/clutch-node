use libp2p::{
    gossipsub::Behaviour as GossipsubBehaviour, mdns::tokio::Behaviour as MsdnBehaviour,
    request_response::cbor::Behaviour as RequestResponseBehavior, swarm::NetworkBehaviour,
};

use serde::{Deserialize, Serialize};

#[derive(NetworkBehaviour)]
pub struct P2PBehaviour {
    pub gossipsub: GossipsubBehaviour,
    pub mdns: MsdnBehaviour,
    pub request_response: RequestResponseBehavior<DirectMessageRequest, DirectMessageResponse>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DirectMessageRequest {
    pub message: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DirectMessageResponse {
    pub message: String,
}
