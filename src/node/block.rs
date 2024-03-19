use sha2::{Sha256, Digest};
use crate::node::transaction::Transaction;

pub struct Block {
    pub index: usize,
    pub previous_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        
        let transactions_hash_string = self.transactions.iter()
            .map(|tx| format!("{}", tx.hash))
            .collect::<Vec<String>>()
            .join(""); 
                   
        hasher.update(format!("{}{}{}", self.index, self.previous_hash, transactions_hash_string));
        let result = hasher.finalize();
        format!("{:x}", result)  
    }

    pub fn new_genesis_block() -> Block {
        let mut genesis_block = Block{
            hash: String::new(),
            index:0,
            previous_hash: "0".to_string(),
            transactions : vec![]
        };

        genesis_block.transactions = Transaction::new_genesis_transactions();
        genesis_block.hash = genesis_block.calculate_hash();
        genesis_block
    }
}
