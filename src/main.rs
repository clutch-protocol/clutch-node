use clap::Parser;
mod node;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[clap(short, long, default_value = "default")]
    env: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let env = &args.env;
    let config = node::config::AppConfig::from_env(&env)?;

    println!("Loaded configuration from env {:?}: {:?}", &env, &config);

    node::libp2p::run(config).await
}
