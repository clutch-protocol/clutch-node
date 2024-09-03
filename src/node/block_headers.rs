use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct BlockHeaders {
    pub block_headers: Vec<BlockHeader>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    pub index: usize,
    pub previous_hash: String,
    pub author: String,
    pub signature_r: String,
    pub signature_s: String,
    pub signature_v: i32,
    pub hash: String,
}

impl BlockHeaders {
    pub fn to_block_indexes(&self) -> Vec<usize>{
         self
            .block_headers
            .iter()
            .map(|header| header.index)
            .collect()
    }
}
