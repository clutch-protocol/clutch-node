use serde::{Deserialize, Serialize};

use super::block::Block;


#[derive(Debug, Serialize, Deserialize)]
pub struct BlockBodies {
    pub blocks: Vec<Block>,
}