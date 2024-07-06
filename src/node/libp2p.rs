use crate::node::blockchain::Blockchain;
use crate::node::config::AppConfig;
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

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
pub struct MyBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

pub async fn run(
    config: &AppConfig,
    blockchain: Arc<Mutex<Blockchain>>,
) -> Result<(), Box<dyn Error>> {
    setup_tracing()?;

    let mut swarm = build_swarm()?;
    let topic = setup_gossipsub_topic(&mut swarm, &config.libp2p_topic_name)?;

    listen_for_connections(&mut swarm)?;
    process_messages(&mut swarm, topic).await
}

fn setup_tracing() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .expect("setup_tracing error");
    Ok(())
}

fn build_swarm() -> Result<Swarm<MyBehaviour>, Box<dyn Error>> {
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

            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            Ok(MyBehaviour { gossipsub, mdns })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();
    Ok(swarm)
}

fn setup_gossipsub_topic(
    swarm: &mut Swarm<MyBehaviour>,
    topic_name: &String,
) -> Result<IdentTopic, Box<dyn Error>> {
    let topic = IdentTopic::new(topic_name);
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
    Ok(topic)
}

fn listen_for_connections(swarm: &mut Swarm<MyBehaviour>) -> Result<(), Box<dyn Error>> {
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    Ok(())
}

async fn process_messages(
    swarm: &mut Swarm<MyBehaviour>,
    topic: IdentTopic,
) -> Result<(), Box<dyn Error>> {
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    println!("Enter messages via STDIN and they will be sent to connected peers using Gossipsub");

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
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(MdnsEvent::Discovered(list))) => {
                    handle_mdns_discovered(swarm, list);
                },
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(MdnsEvent::Expired(list))) => {
                    handle_mdns_expired(swarm, list);
                },
                SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(GossipsubEvent::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                })) => {
                    handle_gossipsub_message(peer_id, id, message);
                },
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Local node is listening on {address}");
                }
                _ => {}
            }
        }
    }
}

fn handle_mdns_discovered(swarm: &mut Swarm<MyBehaviour>, list: Vec<(PeerId, Multiaddr)>) {
    for (peer_id, _multiaddr) in list {
        println!("mDNS discovered a new peer: {peer_id}");
        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
    }
}

fn handle_mdns_expired(swarm: &mut Swarm<MyBehaviour>, list: Vec<(PeerId, Multiaddr)>) {
    for (peer_id, _multiaddr) in list {
        println!("mDNS discover peer has expired: {peer_id}");
        swarm
            .behaviour_mut()
            .gossipsub
            .remove_explicit_peer(&peer_id);
    }
}

fn handle_gossipsub_message(peer_id: PeerId, id: MessageId, message: gossipsub::Message) {
    println!(
        "Got message: '{}' with id: {id} from peer: {peer_id}",
        String::from_utf8_lossy(&message.data),
    );
}
