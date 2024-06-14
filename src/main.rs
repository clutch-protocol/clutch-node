mod node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    node::libp2p::run().await
}
