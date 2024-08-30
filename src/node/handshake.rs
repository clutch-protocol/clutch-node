use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Handshake {
    pub latest_block_hash: String,
}
