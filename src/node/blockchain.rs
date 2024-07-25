use super::config::AppConfig;
use super::consensus::Consensus;
use crate::node::account_state::AccountState;
use crate::node::aura::Aura;
use crate::node::block::Block;
use crate::node::database::Database;
use crate::node::file_utils::write_to_file;
use crate::node::network::Network;
use crate::node::transaction::Transaction;
use crate::node::transaction_pool::TransactionPool;

pub struct Blockchain {
    pub name: String,
    db: Database,
    developer_mode: bool,
    consensus: Aura,
    author_public_key: String,
    author_secret_key: String,
}

impl Blockchain {
    pub fn new(
        name: String,
        author_public_key: String,
        author_secret_key: String,
        developer_mode: bool,
        authorities: Vec<String>,
    ) -> Blockchain {
        let db = Database::new_db(&name);
        let step_duration = 60 / authorities.len() as u64;
        let blockchain = Blockchain {
            name,
            db,
            developer_mode,
            consensus: Aura::new(authorities, step_duration),
            author_public_key,
            author_secret_key,
        };

        Block::genesis_import_block(&blockchain.db);
        blockchain
    }

    pub fn get_latest_block(&self) -> Option<Block> {
        Block::get_latest_block(&self.db)
    }

    pub fn get_account_state(&self, public_key: &String) -> AccountState {
        AccountState::get_current_state(public_key, &self.db)
    }

    pub fn shutdown_blockchain(&mut self) {
        if self.developer_mode {
            self.blockchain_write_to_file();
            self.cleanup_db();
        }
    }

    fn cleanup_db(&mut self) {
        self.db.close();
        match self.db.delete_database(self.name.as_str()) {
            Ok(_) => println!("Developer mode: Database cleaned up successfully."),
            Err(e) => println!("Error cleaning up database: {}", e),
        }
    }

    pub fn import_block(&self, block: &Block) -> Result<(), String> {
        self.consensus.verify_block_author(&block)?;
        block.validate_block(&self.db)?;
        Transaction::validate_transactions(&self.db, &block.transactions)?;
        Block::add_block_to_chain(&self.db, block);

        Ok(())
    }

    pub fn get_blocks(&self) -> Result<Vec<Block>, String> {
        Block::get_blocks(&self.db)
    }

    pub fn current_author(&self) -> &String {
        self.consensus.current_author()
    }

    pub fn add_transaction_to_pool(&self, transaction: &Transaction) -> Result<(), String> {
        transaction.validate_transaction(&self.db)?;
        TransactionPool::add_transaction(&self.db, &transaction)
    }

    pub fn get_transactions_from_pool(&self) -> Result<Vec<Transaction>, String> {
        TransactionPool::get_transactions(&self.db)
    }

    pub fn author_new_block(&self) -> Result<Block, String> {     
        
        let latest_block = match self.get_latest_block() {
            Some(block) => block,
            None => return Err("Failed to get the latest block in author_new_block".to_string()),
        };

        let index = latest_block.index + 1;
        let previous_hash = latest_block.hash;
        let transactions = match TransactionPool::get_transactions(&self.db) {
            Ok(transactions) => transactions,
            Err(e) => return Err(format!("Failed to get transactions from pool: {}", e)),
        };

        let mut new_block = Block::new_block(index, previous_hash, transactions);
        new_block.sign(&self.author_public_key, &self.author_secret_key);
        self.import_block(&new_block)?;
        Ok(new_block)
    }

    pub async fn start_network_services(self, config: &AppConfig) {
        Network::start_services(config, self).await;
    }

    fn blockchain_write_to_file(&mut self) {
        match self.get_blocks() {
            Ok(blocks) => match serde_json::to_string_pretty(&blocks) {
                Ok(json_str) => {
                    let file_name = format!("{}_blockchain_blocks", &self.name);
                    if let Err(e) = write_to_file(&json_str, &file_name) {
                        println!("{}", e);
                    }
                }
                Err(e) => println!("Failed to serialize blocks: {}", e),
            },
            Err(e) => println!("Failed to retrieve blocks: {}", e),
        }

        match self.get_transactions_from_pool() {
            Ok(transactions) => match serde_json::to_string_pretty(&transactions) {
                Ok(json_str) => {
                    let file_name = format!("{}_tx_pool", &self.name);
                    if let Err(e) = write_to_file(&json_str, &file_name) {
                        println!("{}", e);
                    }
                }
                Err(e) => println!("Failed to serialize transactions: {}", e),
            },
            Err(e) => println!("Failed to retrieve transactions in transaction pool: {}", e),
        }
    }
}
