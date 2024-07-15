use crate::node::network::Network;
use crate::node::transaction::Transaction;
use futures::stream::StreamExt;
use libp2p::{
    gossipsub, gossipsub::Event as GossipsubEvent, gossipsub::IdentTopic, gossipsub::MessageId,
    mdns, mdns::Event as MdnsEvent, noise, swarm::NetworkBehaviour, swarm::Swarm,
    swarm::SwarmEvent, tcp, yamux, Multiaddr, PeerId,
};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::{io, io::AsyncBufReadExt, select};
use tracing_subscriber::EnvFilter;

use super::blockchain::Blockchain;

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
    pub fn new(topic_name: &str) -> Self {
        let mut swarm = Self::build_swarm().unwrap();
        let topic = Self::setup_gossipsub_topic(&mut swarm, topic_name).unwrap();

        Self {
            behaviour: swarm,
            topic: topic,
        }
    }

    pub fn broadcast_transaction(&mut self, transaction: &Transaction) {       
        let transaction_data = serde_json::to_string(&transaction).expect("Failed to serialize transaction");
        if let Err(e) = self.behaviour.behaviour_mut().gossipsub.publish(self.topic.clone(), transaction_data) {
            eprintln!("Failed to publish transaction: {}", e);
        }
    } 

    pub async fn run(
        &mut self,        
        blockchain: Arc<Mutex<Blockchain>>,
    ) -> Result<(), Box<dyn Error>> {
        Self::setup_tracing()?;      

        Self::listen_for_connections(&mut self.behaviour)?;
        let topic = self.topic.clone();
        Self::process_messages(&mut self.behaviour, topic, blockchain).await
    }

    fn setup_tracing() -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init()
            .expect("setup_tracing error");
        Ok(())
    }

    fn build_swarm() -> Result<Swarm<P2PBehaviour>, Box<dyn Error>> {
        let swarm = libp2p::SwarmBuilder::with_new_identity()
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
        swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        Ok(())
    }

    async fn process_messages(
        swarm: &mut Swarm<P2PBehaviour>,
        topic: IdentTopic,
        blockchain: Arc<Mutex<Blockchain>>,
    ) -> Result<(), Box<dyn Error>> {
        let mut stdin = io::BufReader::new(io::stdin()).lines();
        println!(
            "Enter messages via STDIN and they will be sent to connected peers using Gossipsub"
        );

        loop {
            select! {
                Ok(Some(line)) = stdin.next_line() => {
                    if let Err(e) = swarm
                        .behaviour_mut()
                        .gossipsub
                        .publish(topic.clone(), line.as_bytes()) {
                        println!("Publish error: {e:?}");
                    }
                }
                event = swarm.select_next_some() => Self::handle_swarm_event(event, swarm, &blockchain).await,
            }
        }
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

        let transaction_result: Result<Transaction, _> = serde_json::from_slice(&message.data);

        if let Ok(transaction) = transaction_result {
            let blockchain =  Arc::clone(&blockchain);
            tokio::task::spawn_blocking(move || {
                if let Ok(blockchain) = blockchain.lock() {
                    if blockchain.add_transaction_to_pool(&transaction).is_ok() {
                        println!("Transaction added to pool from peer: {peer_id}");
                    } else {
                        println!("Failed to add transaction to pool from peer: {peer_id}");
                    }
                }
            })
            .await
            .unwrap();
        } else {
            println!("Failed to deserialize transaction from peer: {peer_id}");
        }
    }
}
