use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBlockHeaders {
    pub start_block_hash: String,
    pub skip : usize,
    pub limit : usize,
}