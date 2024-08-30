use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Handshake {
    pub genesis_block_hash: String,
    pub latest_block_hash: String,
}
