use clap::Parser;
mod node;
use node::blockchain::Blockchain;
use node::libp2p::run;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "default")]
    env: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let env = &args.env;
    let config = node::config::AppConfig::from_env(&env)?;
    println!("Loaded configuration from env {:?}: {:?}", &env, &config);

    let mut blockchain = Blockchain::new(config.blockchain_name.clone(), config.developer_mode.clone(), config.authorities.clone());
    run(config).await
}
