use libp2p::{
    request_response::{Event as RequestResponseEvent, Message as RequestResponseMessage},
    swarm::Swarm,
};

use super::behaviour::{DirectMessageRequest, DirectMessageResponse};
use super::P2PBehaviour;

pub fn handle_request_response(
    event: RequestResponseEvent<DirectMessageRequest, DirectMessageResponse>,
    swarm: &mut Swarm<P2PBehaviour>,
) {
    match event {
        RequestResponseEvent::Message { peer, message } => {
            match message {
                RequestResponseMessage::Request {
                    request_id,
                    request,
                    channel,
                } => {
                    println!(
                        "Received request from {:?} with request_id {:?}: {}",
                        peer,
                        request_id,
                        String::from_utf8_lossy(&request.message)
                    );

                    // Prepare the response
                    let response = DirectMessageResponse {
                        message: format!(
                            "Hello back, {}",
                            String::from_utf8_lossy(&request.message)
                        )
                        .as_bytes()
                        .to_vec(),
                    };

                    swarm
                        .behaviour_mut()
                        .request_response
                        .send_response(channel, response)
                        .expect("Failed to send response");
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
            }
        }
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
