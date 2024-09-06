use std::error::Error;
use tracing_subscriber::EnvFilter;

pub fn setup_tracing(log_level: &str) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_level))
        .try_init()
        .or_else(|_| {
            println!("Global default trace dispatcher has already been set");
            Ok::<(), Box<dyn Error>>(())
        })?;
    Ok(())
}