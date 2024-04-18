use crate::node::blockchain::Blockchain;
use crate::node::transaction::Transaction;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: usize,
    pub previous_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();

        let transactions_hash_string = self
            .transactions
            .iter()
            .map(|tx| format!("{}", tx.hash))
            .collect::<Vec<String>>()
            .join("");

        hasher.update(format!(
            "{}{}{}",
            self.index, self.previous_hash, transactions_hash_string
        ));
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn new_genesis_block() -> Block {
        let mut genesis_block = Block {
            hash: String::new(),
            previous_hash: "0".to_string(),
            index: 0,
            transactions: vec![],
        };

        genesis_block.transactions = Transaction::new_genesis_transactions();
        genesis_block.hash = genesis_block.calculate_hash();
        genesis_block
    }

    pub fn new_block(index: usize, transactions: Vec<Transaction>) -> Block {
        let mut block = Block {
            hash: String::new(),
            previous_hash: "0x".to_string(),
            index: index,
            transactions: transactions,
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn validate_block(&self, blockchain: &Blockchain) -> bool {
        let latest_block_index = blockchain.get_latest_block_index();

        if self.index != latest_block_index + 1 {
            println!(
                "Invalid block: The block index should be {}, but it was {}.",
                latest_block_index + 1,
                self.index
            );
            return false;
        }

        true
    }

    pub fn state_block(&self) -> Option<(Vec<Vec<u8>>,Vec<Vec<u8>>)>{        
        let mut keys: Vec<Vec<u8>> = Vec::new();
        let mut values: Vec<Vec<u8>> = Vec::new();

        //Add block
        let block_key = format!("block_{}", self.index).into_bytes();
        let block_value = serde_json::to_string(self).unwrap().into_bytes();
        keys.push(block_key);
        values.push(block_value);      

        Some((keys,values))
    }

    pub fn state_blockchain(&self) -> Option<(Vec<Vec<u8>>,Vec<Vec<u8>>)>{        
        let mut keys: Vec<Vec<u8>> = Vec::new();
        let mut values: Vec<Vec<u8>> = Vec::new();
       
        // Save the latest block index to the blockchain
        let latest_index_key = b"blockchain_latest_block_index";
        let latest_index_value = self.index.to_string().into_bytes();

        keys.push(latest_index_key.to_vec());
        values.push(latest_index_value);

        Some((keys,values))
    }

}
