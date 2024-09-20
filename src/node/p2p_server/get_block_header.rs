use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBlockHeaders {
    pub start_block_index: usize,
    pub skip : usize,
    pub limit : usize,
}