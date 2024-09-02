use serde::{Deserialize, Serialize};

use super::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockBodies {
    pub block_bodies: Vec<BlockBody>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockBody {
    pub transactions: Vec<Transaction>,
}
