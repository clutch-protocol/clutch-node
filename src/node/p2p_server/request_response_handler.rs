use super::behaviour::{DirectMessageRequest, DirectMessageResponse};
use super::P2PBehaviour;
use crate::node::block_bodies::{BlockBodies, BlockBody};
use crate::node::block_headers::{BlockHeader, BlockHeaders};
use crate::node::blockchain::Blockchain;
use crate::node::get_block_bodies::GetBlockBodies;
use crate::node::get_block_header::GetBlockHeaders;
use crate::node::handshake::Handshake;
use crate::node::p2p_server::commands::DirectMessageType;
use crate::node::rlp_encoding::{decode, encode};
use libp2p::request_response::OutboundRequestId;
use libp2p::{
    request_response::{Event as RequestResponseEvent, Message as RequestResponseMessage},
    swarm::Swarm,
    PeerId,
};
use rlp::Encodable;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_request_response(
    event: RequestResponseEvent<DirectMessageRequest, DirectMessageResponse>,
    swarm: &mut Swarm<P2PBehaviour>,
    blockchain: &Arc<Mutex<Blockchain>>,
) {
    match event {
        RequestResponseEvent::Message { peer, message } => match message {
            RequestResponseMessage::Request {
                request_id,
                request,
                channel,
            } => {
                handle_request_message(peer, request_id, request, channel, swarm, blockchain).await
            }
            RequestResponseMessage::Response {
                request_id,
                response,
            } => handle_response_message(peer, request_id, response).await,
        },
        RequestResponseEvent::OutboundFailure {
            peer,
            request_id,
            error,
        } => {
            eprintln!(
                "Failed to send request to peer {:?} with request_id {:?}: {:?}",
                peer, request_id, error
            );
        }
        RequestResponseEvent::InboundFailure {
            peer,
            request_id,
            error,
        } => {
            eprintln!(
                "Failed to receive request from peer {:?} with request_id {:?}: {:?}",
                peer, request_id, error
            );
        }
        RequestResponseEvent::ResponseSent { peer, request_id } => {
            println!("Response sent to peer {} for request {}", peer, request_id);
        }
    }
}

async fn handle_request_message(
    peer: libp2p::PeerId,
    request_id: libp2p::request_response::InboundRequestId,
    request: DirectMessageRequest,
    channel: libp2p::request_response::ResponseChannel<DirectMessageResponse>,
    swarm: &mut Swarm<P2PBehaviour>,
    blockchain: &Arc<Mutex<Blockchain>>,
) {
    println!(
        "Send direct message from peer:{:?} with id {:?}",
        peer, request_id,
    );

    let message_type = DirectMessageType::from_byte(request.message[0]);
    let payload = &request.message[1..];

    let response_message = match message_type {
        Some(DirectMessageType::Handshake) => handle_handshake_request(payload, blockchain).await,
        Some(DirectMessageType::GetBlockHeaders) => {
            handle_get_block_headers_request(payload, blockchain).await
        }
        Some(DirectMessageType::GetBlockBodies) => {
            handle_get_block_bodies_request(payload, blockchain).await
        }
        _ => {
            eprintln!(
                "Received unknown DirectMessageType from peer {:?}: {:?}",
                peer, message_type
            );
            return;
        }
    };

    send_response(response_message, swarm, channel);
}

async fn handle_response_message(
    peer: libp2p::PeerId,
    request_id: libp2p::request_response::OutboundRequestId,
    response: DirectMessageResponse,
) {
    println!(
        "Received direct message response from {:?} with request_id {:?}",
        peer, request_id,
    );

    let message_type = DirectMessageType::from_byte(response.message[0]);
    let payload = &response.message[1..];

    match message_type {
        Some(DirectMessageType::Handshake) => handle_handshake_response(payload),
        Some(DirectMessageType::BlockHeaders) => handle_block_headers_response(payload),
        Some(DirectMessageType::BlockBodies) => handle_block_bodies_response(payload),
        _ => {
            eprintln!(
                "Unknown DirectMessageType in response from peer {:?}: {:?}",
                peer, message_type
            );
        }
    }
}

fn send_request(
    peer_id: &PeerId,
    request_message: Vec<u8>,
    swarm: &mut Swarm<P2PBehaviour>,
) -> OutboundRequestId {
    let request: DirectMessageRequest = DirectMessageRequest {
        message: request_message,
    };

    swarm
        .behaviour_mut()
        .request_response
        .send_request(&peer_id, request)
}

fn send_response(
    response_message: Vec<u8>,
    swarm: &mut Swarm<P2PBehaviour>,
    channel: libp2p::request_response::ResponseChannel<DirectMessageResponse>,
) {
    let response = DirectMessageResponse {
        message: response_message,
    };

    if let Err(e) = swarm
        .behaviour_mut()
        .request_response
        .send_response(channel, response)
    {
        eprintln!("Failed to send response: {:?}", e);
    }
}

async fn handle_handshake_request(payload: &[u8], blockchain: &Arc<Mutex<Blockchain>>) -> Vec<u8> {
    match decode::<Handshake>(payload) {
        Ok(handshake) => {
            println!("Received and decoded handshake: {:?}", handshake);
            handshake_response(&handshake, blockchain).await
        }
        Err(e) => {
            eprintln!("Failed to decode handshake: {:?}", e);
            Vec::new()
        }
    }
}

async fn handle_get_block_headers_request(
    payload: &[u8],
    blockchain: &Arc<Mutex<Blockchain>>,
) -> Vec<u8> {
    match decode::<GetBlockHeaders>(payload) {
        Ok(get_block_header) => {
            println!(
                "Received and decoded getBlockHeader: {:?}",
                get_block_header
            );
            get_block_headers_response(&get_block_header, blockchain).await
        }
        Err(e) => {
            eprintln!("Failed to decode getBlockHeader: {:?}", e);
            Vec::new()
        }
    }
}

async fn handle_get_block_bodies_request(
    payload: &[u8],
    blockchain: &Arc<Mutex<Blockchain>>,
) -> Vec<u8> {
    match decode::<GetBlockBodies>(payload) {
        Ok(get_block_bodies) => {
            println!(
                "Received and decoded GetBlockBodies: {:?}",
                get_block_bodies
            );
            get_block_bodies_response(&get_block_bodies, blockchain).await
        }
        Err(e) => {
            eprintln!("Failed to decode GetBlockBodies: {:?}", e);
            Vec::new()
        }
    }
}

fn handle_handshake_response(payload: &[u8]) {
    match decode::<Handshake>(payload) {
        Ok(handshake) => {
            println!("Decoded Handshake: {:?}", handshake);
        }
        Err(e) => {
            eprintln!("Failed to decode Handshake: {:?}", e);
        }
    }
}

fn handle_block_headers_response(payload: &[u8]) {
    match decode::<BlockHeaders>(payload) {
        Ok(block_headers) => {
            println!("Decoded BlockHeaders: {:?}", block_headers);
        }
        Err(e) => {
            eprintln!("Failed to decode BlockHeaders: {:?}", e);
        }
    }
}

fn handle_block_bodies_response(payload: &[u8]) {
    match decode::<BlockBodies>(payload) {
        Ok(block_bodies) => {
            println!("Decoded BlockBodies: {:?}", block_bodies);
        }
        Err(e) => {
            eprintln!("Failed to decode BlockBodies: {:?}", e);
        }
    }
}

async fn handshake_response(
    _handshake: &Handshake,
    blockchain: &Arc<Mutex<Blockchain>>,
) -> Vec<u8> {
    let blockchain = blockchain.lock().await;
    let response_handshake = blockchain
        .handshake()
        .expect("error get handshake response");
    encode_message(DirectMessageType::Handshake, &response_handshake)
}

async fn get_block_headers_response(
    get_block_header: &GetBlockHeaders,
    blockchain: &Arc<Mutex<Blockchain>>,
) -> Vec<u8> {
    let blockchain = blockchain.lock().await;
    let blocks = blockchain
        .get_blocks_with_limit_and_skip(
            get_block_header.start_block_index,
            get_block_header.skip,
            get_block_header.limit,
        )
        .expect("Failed to get blocks");

    let block_headers: Vec<BlockHeader> =
        blocks.iter().map(|block| block.to_block_header()).collect();

    let response_block_headers = BlockHeaders { block_headers };
    encode_message(DirectMessageType::BlockHeaders, &response_block_headers)
}

async fn get_block_bodies_response(
    get_block_bodies: &GetBlockBodies,
    blockchain: &Arc<Mutex<Blockchain>>,
) -> Vec<u8> {
    let blockchain = blockchain.lock().await;
    let blocks = blockchain
        .get_blocks_by_indexes(get_block_bodies.block_indexes.clone())
        .expect("Failed to get blocks");

    let block_bodies: Vec<BlockBody> = blocks.iter().map(|block| block.to_block_body()).collect();

    let response_block_bodies = BlockBodies { block_bodies };
    encode_message(DirectMessageType::BlockBodies, &response_block_bodies)
}

fn encode_message<T: serde::Serialize + Encodable>(
    message_type: DirectMessageType,
    message: &T,
) -> Vec<u8> {
    let encoded_message = encode(message);
    let mut message_with_type = Vec::with_capacity(1 + encoded_message.len());
    message_with_type.push(message_type.as_byte());
    message_with_type.extend(encoded_message);
    message_with_type
}
