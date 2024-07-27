use crate::node::block::Block;
use crate::node::blockchain::Blockchain;
use crate::node::rlp_encoding::decode;
use crate::node::transaction::Transaction;
use futures::stream::StreamExt;
use futures::FutureExt;
use libp2p::{
    gossipsub, gossipsub::Event as GossipsubEvent, gossipsub::IdentTopic, gossipsub::MessageId,
    mdns, mdns::Event as MdnsEvent, noise, swarm::NetworkBehaviour, swarm::Swarm,
    swarm::SwarmEvent, tcp, yamux, Multiaddr, PeerId,
};

use std::collections::hash_map::DefaultHasher;
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
}

pub struct P2PServer {
    pub behaviour: Swarm<P2PBehaviour>,
    pub topic: IdentTopic,
}

impl P2PServer {
    pub fn new(topic_name: &str, listen_addrs: &[&str]) -> Result<Self, Box<dyn Error>> {
        let mut swarm = Self::build_swarm(listen_addrs)?;
        let topic = Self::setup_gossipsub_topic(&mut swarm, topic_name)?;

        Ok(Self {
            behaviour: swarm,
            topic,
        })
    }

    pub async fn gossip_message(
        command_tx_p2p: Sender<P2PServerCommand>,
        message_type: MessageType,
        message: &Vec<u8>,
    ) {
        let mut message_with_type = vec![message_type.as_byte()];
        message_with_type.extend(message);

        let (response_tx, response_rx) = oneshot::channel();
        command_tx_p2p
            .send(P2PServerCommand::SendMessage {
                message: message_with_type,
                response_tx,
            })
            .await
            .unwrap();

        match response_rx.await {
            Ok(result) => match result {
                Ok(message_id) => println!("Message sent with id: {:?}", message_id),
                Err(e) => eprintln!("Failed to send message: {:?}", e),
            },
            Err(e) => eprintln!("Failed to receive response: {:?}", e),
        }
    }

    pub async fn run(
        &mut self,
        blockchain: Arc<Mutex<Blockchain>>,
        mut command_rx: tokio::sync::mpsc::Receiver<P2PServerCommand>,
    ) -> Result<(), Box<dyn Error>> {
        Self::setup_tracing()?;
        Self::listen_for_connections(&mut self.behaviour)?;
        self.process_messages(blockchain, &mut command_rx).await
    }

    fn setup_tracing() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init()
            .or_else(|_| {
                println!("Global default trace dispatcher has already been set");
                Ok::<(), Box<dyn Error>>(())
            })?;
        Ok(())
    }

    fn build_swarm(listen_addrs: &[&str]) -> Result<Swarm<P2PBehaviour>, Box<dyn Error>> {
        let mut swarm = libp2p::SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_quic()
            .with_behaviour(|key| {
                let message_id_fn = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    message.data.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                };

                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(10))
                    .validation_mode(gossipsub::ValidationMode::Strict)
                    .message_id_fn(message_id_fn)
                    .build()
                    .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?;

                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )?;

                let mdns = mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?;
                Ok(P2PBehaviour { gossipsub, mdns })
            })?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        for addr in listen_addrs {
            swarm.listen_on(addr.parse()?)?;
        }

        Ok(swarm)
    }

    fn setup_gossipsub_topic(
        swarm: &mut Swarm<P2PBehaviour>,
        topic_name: &str,
    ) -> Result<IdentTopic, Box<dyn Error>> {
        let topic = IdentTopic::new(topic_name);
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
        Ok(topic)
    }

    fn listen_for_connections(swarm: &mut Swarm<P2PBehaviour>) -> Result<(), Box<dyn Error>> {
        //swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        Ok(())
    }

    async fn process_messages(
        &mut self,
        blockchain: Arc<Mutex<Blockchain>>,
        command_rx: &mut tokio::sync::mpsc::Receiver<P2PServerCommand>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            select! {
                event = self.behaviour.select_next_some().fuse() => {
                    Self::handle_swarm_event(event, &mut self.behaviour, &blockchain).await;
                },
                command = command_rx.recv() => {
                    if let Some(command) = command {
                        match command {
                            P2PServerCommand::SendMessage { message, response_tx } => {
                                let result = self.send_gossip_message(message);
                                let _ = response_tx.send(result);
                            }
                        }
                    }
                },
            }
        }
    }

    fn send_gossip_message(
        &mut self,
        message: Vec<u8>,
    ) -> Result<MessageId, gossipsub::PublishError> {
        self.behaviour
            .behaviour_mut()
            .gossipsub
            .publish(self.topic.clone(), message)
    }

    async fn handle_swarm_event(
        event: SwarmEvent<P2PBehaviourEvent>,
        swarm: &mut Swarm<P2PBehaviour>,
        blockchain: &Arc<Mutex<Blockchain>>,
    ) {
        match event {
            SwarmEvent::Behaviour(P2PBehaviourEvent::Mdns(MdnsEvent::Discovered(list))) => {
                Self::handle_mdns_discovered(swarm, list);
            }
            SwarmEvent::Behaviour(P2PBehaviourEvent::Mdns(MdnsEvent::Expired(list))) => {
                Self::handle_mdns_expired(swarm, list);
            }
            SwarmEvent::Behaviour(P2PBehaviourEvent::Gossipsub(GossipsubEvent::Message {
                propagation_source: peer_id,
                message_id: id,
                message,
            })) => {
                Self::handle_gossipsub_message(peer_id, id, message, blockchain).await;
            }
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Local node is listening on {address}");
            }
            _ => {}
        }
    }

    fn handle_mdns_discovered(swarm: &mut Swarm<P2PBehaviour>, list: Vec<(PeerId, Multiaddr)>) {
        for (peer_id, _multiaddr) in list {
            println!("mDNS discovered a new peer: {peer_id}");
            swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
        }
    }

    fn handle_mdns_expired(swarm: &mut Swarm<P2PBehaviour>, list: Vec<(PeerId, Multiaddr)>) {
        for (peer_id, _multiaddr) in list {
            println!("mDNS discover peer has expired: {peer_id}");
            swarm
                .behaviour_mut()
                .gossipsub
                .remove_explicit_peer(&peer_id);
        }
    }

    async fn handle_gossipsub_message(
        peer_id: PeerId,
        id: MessageId,
        message: gossipsub::Message,
        blockchain: &Arc<Mutex<Blockchain>>,
    ) {
        println!(
            "Got message: '{}' with id: {id} from peer: {peer_id}",
            String::from_utf8_lossy(&message.data),
        );

        let message_type = MessageType::from_byte(message.data[0]);
        let payload = &message.data[1..];

        match message_type {
            Some(MessageType::Transaction) => match decode::<Transaction>(payload) {
                Ok(transaction) => {
                    println!("Decoded transaction: {:?}", &transaction);
                    handle_received_transaction(&transaction, blockchain).await;
                }
                Err(e) => {
                    eprintln!("Failed to decode transaction: {:?}", e);
                }
            },
            Some(MessageType::Block) => match decode::<Block>(payload) {
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

pub enum P2PServerCommand {
    SendMessage {
        message: Vec<u8>,
        response_tx: tokio::sync::oneshot::Sender<Result<MessageId, gossipsub::PublishError>>,
    },
}

#[derive(Debug)]
pub enum MessageType {
    Transaction,
    Block,
}

impl MessageType {
    fn as_byte(&self) -> u8 {
        match self {
            MessageType::Transaction => 0x01,
            MessageType::Block => 0x02,
        }
    }

    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(MessageType::Transaction),
            0x02 => Some(MessageType::Block),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_p2p_server_gossip_message() {
        let topic_name = "test-topic";

        // Create two P2P servers
        let mut server1 = P2PServer::new(topic_name, &["/ip4/127.0.0.1/tcp/4001"]).unwrap();
        let mut server2 = P2PServer::new(topic_name, &["/ip4/127.0.0.1/tcp/4002"]).unwrap();

        // Set up blockchain instances

        let blockchain = Arc::new(Mutex::new(initialize_blockchain(
            "clutch-node-test-1".to_string(),
        )));
        let b1 = Arc::clone(&blockchain);
        let b2 = Arc::clone(&blockchain);
        // Set up command channels
        let (command_tx1, command_rx1) = mpsc::channel(32);
        let (command_tx2, command_rx2) = mpsc::channel(32);

        // Run servers in the background
        tokio::spawn(async move {
            server1.run(b1, command_rx1).await.unwrap();
        });
        tokio::spawn(async move {
            server2.run(b2, command_rx2).await.unwrap();
        });

        // Wait for servers to start
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Send a message from server1 to server2
        let message = b"Hello, world!".to_vec();
        P2PServer::gossip_message(command_tx1.clone(), MessageType::Transaction, &message).await;

        // Wait for the message to propagate
        tokio::time::sleep(Duration::from_secs(5)).await;

        // Check if the message was received by server2
        // This part depends on how you want to validate the message reception.
        // For simplicity, we're printing the message in the handle_gossipsub_message method.
        // You can add a flag or counter to verify it here.

        // Shut down the servers
        drop(command_tx1);
        drop(command_tx2);
        let b3 = Arc::clone(&blockchain);
        b3.lock().await.shutdown_blockchain();
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
}
