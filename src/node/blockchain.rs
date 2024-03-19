use crate::node::block::Block; 

pub struct Blockchain {
    pub name: &'static str,
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {            
        let genesis_block = Block::new_genesis_block();
        
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