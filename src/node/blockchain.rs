use sha2::{Sha256, Digest};

pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,  // Use the appropriate data type for your use case
}

pub struct Block {
    pub index: usize,
    pub previous_hash: String,
    pub hash: String,
    pub transactions : Vec<Transaction>
}

impl Block {   
       // New function to create a hash for the Block
       pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        // Convert transaction data to a string format for hashing
        let transactions_string = self.transactions.iter()
            .map(|tx| format!("{}{}{}", tx.sender, tx.receiver, tx.amount))
            .collect::<Vec<String>>()
            .join("");  // Concatenate all transactions into a single string
        hasher.update(format!("{}{}{}", self.index, self.previous_hash, transactions_string));
        let result = hasher.finalize();
        format!("{:x}", result)  // Converts hash bytes to hex string
    }
}

pub struct Blockchain {
    pub name: &'static str,
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        
        let mut genesis_block = Block{
            index:0,
            previous_hash: "0".to_string(),
            hash: String::new(),
            transactions : vec![]
        };

        genesis_block.hash = genesis_block.calculate_hash();

        let blockchain = Blockchain {
            name: "clutch",      
            blocks: vec![genesis_block],
        };

        blockchain
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn new_blockchain_has_name_clutch() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.name, "clutch", "New blockchain should be named 'clutch'");
    }  

    #[test]
    fn new_blockchain_has_valid_genesis_block() {
        let blockchain = Blockchain::new();
    
        // Check that the blockchain has exactly one block (the genesis block)
        assert_eq!(blockchain.blocks.len(), 1, "The new blockchain should start with exactly one block.");
    
        // Check the properties of the genesis block
        let genesis_block = &blockchain.blocks[0];
        assert_eq!(genesis_block.index, 0, "The index of the genesis block should be 0.");
        assert_eq!(genesis_block.previous_hash, "0", "The previous hash of the genesis block should be '0'.");
        assert_eq!(genesis_block.hash, "f1534392279bddbf9d43dde8701cb5be14b82f76ec6607bf8d6ad557f60f304e", "The hash of the genesis block should be 'f1534392279bddbf9d43dde8701cb5be14b82f76ec6607bf8d6ad557f60f304e', but was '{}'.", genesis_block.hash);
    }
    
}