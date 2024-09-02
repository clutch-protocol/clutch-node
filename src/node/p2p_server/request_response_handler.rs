use super::behaviour::{DirectMessageRequest, DirectMessageResponse};
use super::P2PBehaviour;
use crate::node::block_headers::{BlockHeader, BlockHeaders};
use crate::node::blockchain::Blockchain;
use crate::node::get_block_header::GetBlockHeaders;
use crate::node::handshake::Handshake;
use crate::node::p2p_server::commands::DirectMessageType;
use crate::node::rlp_encoding::{decode, encode};
use libp2p::{
    request_response::{Event as RequestResponseEvent, Message as RequestResponseMessage},
    swarm::Swarm,
};
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
                println!(
                    "Received direct message from peer:{:?} with id {:?}",
                    peer, request_id,
                );

                let message_type = DirectMessageType::from_byte(request.message[0]);
                let payload = &request.message[1..];

                match message_type {
                    Some(DirectMessageType::Handshake) => match decode::<Handshake>(payload) {
                        Ok(handshake) => {
                            println!("Received and decoded handshake: {:?}", &handshake);
                            let response_message = handshake_response(&handshake, blockchain).await;
                            send_message(response_message, swarm, channel);
                        }
                        Err(e) => {
                            eprintln!("Failed to decode handshake: {:?}", e);
                        }
                    },
                    Some(DirectMessageType::GetBlockHeaders) => {
                        match decode::<GetBlockHeaders>(payload) {
                            Ok(get_block_header) => {
                                println!(
                                    "Received and decoded getBlockHeader: {:?}",
                                    &get_block_header
                                );
                                let response_message =
                                    get_block_header_response(&get_block_header, blockchain).await;
                                send_message(response_message, swarm, channel);
                            }
                            Err(e) => {
                                eprintln!("Failed to decode handshake: {:?}", e);
                            }
                        }
                    }
                    _ => {
                        eprintln!(
                            "Received direct message: unknown DirectMessageType: {:?}",
                            message_type
                        );
                    }
                }
            }
            RequestResponseMessage::Response {
                request_id,
                response,
            } => {
                println!(
                    "Received response from {:?} with request_id {:?}",
                    peer, request_id,
                );

                let message_type = DirectMessageType::from_byte(response.message[0]);
                let payload = &response.message[1..];

                match message_type {
                    Some(DirectMessageType::Handshake) => match decode::<Handshake>(payload) {
                        Ok(handshake) => {
                            println!("Decoded handshake: {:?}", &handshake);
                        }
                        Err(e) => {
                            eprintln!("Failed to decode handshake: {:?}", e);
                        }
                    },
                    Some(DirectMessageType::GetBlockHeaders) => {
                        match decode::<BlockHeaders>(payload) {
                            Ok(block_headers) => {
                                println!("Decoded get_block_headers: {:?}", &block_headers);
                            }
                            Err(e) => {
                                eprintln!("Failed to decode get_block_headers: {:?}", e);
                            }
                        }
                    }
                    _ => {
                        eprintln!("Unknown DirectMessageType: {:?}", message_type);
                    }
                }
            }
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

fn send_message(
    response_message: Vec<u8>,
    swarm: &mut Swarm<P2PBehaviour>,
    channel: libp2p::request_response::ResponseChannel<DirectMessageResponse>,
) {
    let response = DirectMessageResponse {
        message: response_message,
    };

    swarm
        .behaviour_mut()
        .request_response
        .send_response(channel, response)
        .expect("Failed to send response");
}

async fn handshake_response(
    _handshake: &Handshake,
    blockchain: &Arc<Mutex<Blockchain>>,
) -> Vec<u8> {
    let blockchain = blockchain.lock().await;
    let latest_block = blockchain
        .get_latest_block()
        .expect("Failed to get latest block");
    let genesis_block = blockchain
        .get_genesis_block()
        .expect("Failed to get genesis block");

    let handshake = Handshake {
        genesis_block_hash: genesis_block.hash,
        latest_block_hash: latest_block.hash,
        latest_block_index: latest_block.index,
    };

    let encoded_handshake = encode(&handshake);

    let mut message_with_type = Vec::with_capacity(1 + encoded_handshake.len());
    message_with_type.push(DirectMessageType::Handshake.as_byte());
    message_with_type.extend(encoded_handshake);

    message_with_type
}

async fn get_block_header_response(
    get_block_header: &GetBlockHeaders,
    blockchain: &Arc<Mutex<Blockchain>>,
) -> Vec<u8> {
    let blockchain = blockchain.lock().await;

    let start_index = get_block_header.start_block_index;
    let skip = get_block_header.skip;
    let limit = get_block_header.limit;
    let blocks = blockchain
        .get_blocks_with_limit_and_skip(start_index, skip, limit)
        .expect("Failed to get blocks");

    let block_headers: Vec<BlockHeader> =
        blocks.iter().map(|block| block.to_block_header()).collect();

    let block_headers = BlockHeaders { block_headers };
    let encoded_block_headers = encode(&block_headers);

    let mut message_with_type = Vec::with_capacity(1 + encoded_block_headers.len());
    message_with_type.push(DirectMessageType::GetBlockHeaders.as_byte());
    message_with_type.extend(encoded_block_headers);

    message_with_type
}
