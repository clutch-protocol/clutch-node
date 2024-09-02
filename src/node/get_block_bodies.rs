use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBlockBodies {
    pub block_indexes: Vec<usize>,        
}