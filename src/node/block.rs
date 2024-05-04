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

    pub fn new_block(index: usize, previous_hash: String, transactions: Vec<Transaction>) -> Block {
        let mut block = Block {
            hash: String::new(),
            previous_hash: previous_hash,
            index: index,
            transactions: transactions,
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn validate_block(&self, blockchain: &Blockchain) -> bool {
        match blockchain.get_latest_block() {
            Some(latest_block) => {
                if self.index != latest_block.index + 1 {
                    println!(
                        "Invalid block: The block index should be {}, but it was {}.",
                        latest_block.index + 1,
                        self.index
                    );
                    return false;
                }

                if self.previous_hash != latest_block.hash {
                    println!(
                        "Invalid block: The previous hash should be {}, but it was {}.",
                        latest_block.hash, self.previous_hash
                    );
                    return false;
                }

                return true;
            }
            None => true,
        }
    }

    pub fn state_block(&self) -> Option<(Vec<Vec<u8>>, Vec<Vec<u8>>)> {
        let mut keys: Vec<Vec<u8>> = Vec::new();
        let mut values: Vec<Vec<u8>> = Vec::new();

        //Add block
        let block_key = format!("block_{}", self.index).into_bytes();
        let block_value = serde_json::to_string(self).unwrap().into_bytes();
        keys.push(block_key);
        values.push(block_value);

        Some((keys, values))
    }

    pub fn state_blockchain(&self) -> Option<(Vec<Vec<u8>>, Vec<Vec<u8>>)> {
        let mut keys: Vec<Vec<u8>> = Vec::new();
        let mut values: Vec<Vec<u8>> = Vec::new();

        // Save the latest block index to the blockchain
        let blockchain_latest_block_key = b"blockchain_latest_block";
        let blockchain_latest_block_value = serde_json::to_string(self).unwrap().into_bytes();

        keys.push(blockchain_latest_block_key.to_vec());
        values.push(blockchain_latest_block_value);

        Some((keys, values))
    }
}
