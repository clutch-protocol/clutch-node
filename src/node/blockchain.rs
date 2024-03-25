use crate::node::block::Block;
use crate::node::database::Database;
use crate::node::transaction::Transaction;

#[derive(Debug)]
pub struct Blockchain {
    pub name: String,
    db: Database,
}

impl Blockchain {
    pub fn new(name: String) -> Blockchain {
        let db = Database::new_db(&name);
        let mut blockchain = Blockchain { name: name, db: db };

        blockchain.ensure_genesis_block_exists();
        blockchain
    }

    fn ensure_genesis_block_exists(&mut self) {
        match self.db.get(b"block_0") {
            Ok(Some(_)) => {
                println!("Genesis block already exists.");
            }
            Ok(None) => {
                println!("Genesis block does not exist, creating new one...");
                let genesis_block = Block::new_genesis_block();
                let serialized_block = serde_json::to_string(&genesis_block).unwrap();
                match self.db.put(b"block_0", serialized_block.as_bytes()) {
                    Ok(_) => println!("Genesis block created and stored successfully."),
                    Err(e) => panic!("Failed to store genesis block: {}", e),
                }
            }
            Err(e) => panic!("Failed to check for genesis block: {}", e),
        }
    }   

    pub fn block_import(&mut self, block: Block) {
        let is_valid_block = Transaction::validate_transactions(&block.transactions);

        if !is_valid_block {
            println!("Block contains invalid transactions and will not be added.");
            return;
        }

        // If all transactions are valid, proceed with adding the block
        println!("All transactions are valid. Block can be added.");
        // Add block to blockchain logic here
        // e.g., self.blocks.push(block); or storing in DB

        // self.blocks.push(block);
    }
}
