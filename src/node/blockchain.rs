use crate::node::account_state::AccountState;
use crate::node::aura::Aura;
use crate::node::block::Block;
use crate::node::database::Database;

use super::consensus::Consensus;

#[derive(Debug)]
pub struct Blockchain {
    pub name: String,
    db: Database,
    developer_mode: bool,
    consensus: Aura,
}

impl Blockchain {
    pub fn new(name: String, developer_mode: bool, authorities: Vec<String>) -> Blockchain {
        let db = Database::new_db(&name);
        let step_duration = 60 / authorities.len() as u64;
        let blockchain = Blockchain {
            name,
            db,
            developer_mode,
            consensus: Aura::new(authorities, step_duration),
        };

        Block::genesis_block_import(&blockchain.db);
        blockchain
    }

    pub fn get_latest_block(&self) -> Option<Block> {
        Block::get_latest_block(&self.db)
    }

    pub fn get_account_state(&self, public_key: &String) -> AccountState {
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
        if !self.consensus.verify_block_author(&block) {
            return Err(String::from("Block author is invalid."));
        }

        if !block.validate_block(&self.db) {
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

        Block::add_block_to_chain(&self.db, block);

        Ok(())
    }

    pub fn get_blocks(&self) -> Result<Vec<Block>, String> {
        Block::get_blocks(&self.db)
    }

    pub fn current_author(&self) -> &String {
        self.consensus.current_author()
    }
}
