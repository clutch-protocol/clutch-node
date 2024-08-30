use super::behaviour::{DirectMessageRequest, DirectMessageResponse};
use super::P2PBehaviour;
use crate::node::blockchain::Blockchain;
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
                    "Received direct message from peer:{:?} with id {:?}: {}",
                    peer,
                    request_id,
                    String::from_utf8_lossy(&request.message)
                );

                let message_type = DirectMessageType::from_byte(request.message[0]);
                let payload = &request.message[1..];

                match message_type {
                    Some(DirectMessageType::Handshake) => match decode::<Handshake>(payload) {
                        Ok(handshake) => {
                            println!("Decoded handshake: {:?}", &handshake);
                            let response_message = handshake_response(&handshake, blockchain).await;
                            send_response(response_message, swarm, channel);
                        }
                        Err(e) => {
                            eprintln!("Failed to decode handshake: {:?}", e);
                        }
                    },
                    _ => {
                        eprintln!("Unknown DirectMessageType: {:?}", message_type);
                    }
                }
            }
            RequestResponseMessage::Response {
                request_id,
                response,
            } => {
                println!(
                    "Received response from {:?} with request_id {:?}: {:?}",
                    peer,
                    request_id,
                    String::from_utf8_lossy(&response.message)
                );
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

fn send_response(
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
    let latest_block = blockchain.get_latest_block().unwrap();

    let handshake = Handshake {
        latest_block_hash: latest_block.hash,
    };

    encode(&handshake)
}
