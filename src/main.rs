mod node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = node::config::AppConfig::from_env()?;
    println!("Loaded configuration: {:?}", config);

    node::libp2p::run().await
}
