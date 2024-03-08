use sha2::{Sha256, Digest};

pub struct Block {
    pub index: usize,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    // New function to create a hash for the Block
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", self.index, self.previous_hash, "Your data here"));
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
        assert_eq!(genesis_block.hash, "525cfeaabe945e2ad405b6e881c8206582b7054d7165a3a15af9fdf9e2b0d56e", "The hash of the genesis block should be '525cfeaabe945e2ad405b6e881c8206582b7054d7165a3a15af9fdf9e2b0d56e', but was '{}'.", genesis_block.hash);
    }
    
}