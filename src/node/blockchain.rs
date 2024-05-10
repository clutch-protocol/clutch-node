use crate::node::block::Block;
use crate::node::database::Database;

use super::account_state::AccountState;

#[derive(Debug)]
pub struct Blockchain {
    pub name: String,
    db: Database,
    developer_mode: bool,
}

impl Blockchain {
    pub fn new(name: String, developer_mode: bool) -> Blockchain {
        let db = Database::new_db(&name);
        let mut blockchain = Blockchain {
            name: name,
            db: db,
            developer_mode: developer_mode,
        };

        blockchain.genesis_block_import();
        blockchain
    }

    pub fn get_latest_block(&self) -> Option<Block> {
        Block::get_latest_block(&self.db)
    }

    pub fn get_current_state(&self, public_key: &String) -> AccountState {
        AccountState::get_current_state(public_key, &self.db)
    }

    pub fn cleanup_if_developer_mode(&mut self) {
        if self.developer_mode {
            self.db.close();
            match self.db.delete_database(self.name.as_str()) {
                Ok(_) => println!("Developer mode: Database cleaned up successfully."),
                Err(e) => println!("Error cleaning up database: {}", e),
            }
        }
    }

    pub fn block_import(&mut self, block: &Block) -> Result<(), String> {
        let is_valid_block = block.validate_block(&self.db);
        if !is_valid_block {
            return Err(String::from("Block is invalid and will not be added."));
        }

        for tx in block.transactions.iter() {
            let is_valid_tx = tx.validate_transaction(&self.db);
            if !is_valid_tx {
                return Err(String::from(
                    "Block contains invalid transactions and will not be added.",
                ));
            }
        }

        self.add_block_to_chain(block);

        Ok(())
    }

    pub fn get_blocks(&self) -> Result<Vec<Block>, String> {
        match self.db.get_keys_values_by_cf_name("block") {
            Ok(entries) => {
                let mut blocks = Vec::new();

                for (_key, value) in entries {
                    match serde_json::from_slice::<Block>(&value) {
                        Ok(block) => {
                            blocks.push(block);
                        }
                        Err(e) => {
                            return Err(format!("Failed to deserialize block: {}", e));
                        }
                    }
                }

                Ok(blocks)
            }
            Err(e) => Err(format!("Failed to retrieve blocks: {}", e)),
        }
    }

    fn genesis_block_import(&mut self) {
        match self.db.get("block", b"block_0") {
            Ok(Some(_)) => {
                println!("Genesis block already exists.");
            }
            Ok(None) => {
                println!("Genesis block does not exist, creating new one...");
                let genesis_block = Block::new_genesis_block();
                self.add_block_to_chain(&genesis_block);
            }
            Err(e) => panic!("Failed to check for genesis block: {}", e),
        }
    }

    fn add_block_to_chain(&mut self, block: &Block) {
        // Storage for keys and values
        let mut cf_storage: Vec<String> = Vec::new();
        let mut keys_storage: Vec<Vec<u8>> = Vec::new();
        let mut values_storage: Vec<Vec<u8>> = Vec::new();

        let mut operations: Vec<(&str, &[u8], &[u8])> = Vec::new();

        // Handle block state
        if let Some((block_keys, block_values)) = block.state_block() {
            for (key, value) in block_keys.into_iter().zip(block_values.into_iter()) {
                cf_storage.push("block".to_string());
                keys_storage.push(key);
                values_storage.push(value);
            }
        } else {
            println!("Failed to serialize block for storage.");
            return;
        }

        // Handle Blockchain State
        if let Some((block_keys, block_values)) = block.state_blockchain() {
            for (key, value) in block_keys.into_iter().zip(block_values.into_iter()) {
                cf_storage.push("blockchain".to_string());
                keys_storage.push(key);
                values_storage.push(value);
            }
        } else {
            println!("Failed to serialize block for storage.");
            return;
        }

        // Handle transactions State
        for tx in block.transactions.iter() {
            let updates = tx.state_transaction(&self.db);

            for update in updates {
                if let Some((key, value)) = update {
                    cf_storage.push("state".to_string());
                    keys_storage.push(key);
                    values_storage.push(value);
                }
            }
        }

        // Prepare operations for database write
        for (key, cf_name) in keys_storage
            .iter()
            .zip(values_storage.iter())
            .zip(cf_storage.iter())
        {
            operations.push((cf_name, key.0, key.1));
        }

        // Update the database
        match self.db.write(operations) {
            Ok(_) => println!(
                "add_block_to_chain successfully. block hash: {}. block index: {}",
                block.hash, block.index
            ),
            Err(e) => panic!("Failed add_block_to_chain: {}", e),
        }
    }
}
