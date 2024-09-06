use tracing::{error, info};

use super::configuration::AppConfig;
use super::consensus::Consensus;
use super::handshake::Handshake;
use crate::node::account_state::AccountState;
use crate::node::aura::Aura;
use crate::node::block::Block;
use crate::node::database::Database;
use crate::node::file_utils::write_to_file;
use crate::node::node_services::NodeServices;
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

    pub fn get_genesis_block(&self) -> Option<Block> {
        Block::get_genesis_block(&self.db)
    }

    #[allow(dead_code)]
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
            Ok(_) => {
                info!("Developer mode: Database cleaned up successfully.");               
            }
            Err(e) => error!("Error cleaning up database: {}", e),
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

    pub fn get_blocks_with_limit_and_skip(
        &self,
        start_index: usize,
        skip: usize,
        limit: usize,
    ) -> Result<Vec<Block>, String> {
        Block::get_blocks_with_limit_and_skip(&self.db, start_index, skip, limit)
    }

    pub fn get_blocks_by_indexes(&self, indexes: Vec<usize>) -> Result<Vec<Block>, String> {
        Block::get_blocks_by_indexes(&self.db, indexes)
    }

    #[allow(dead_code)]
    pub fn current_author(&self) -> &String {
        self.consensus.current_author()
    }

    pub fn handshake(&self) -> Result<Handshake, String> {
        let latest_block = self
            .get_latest_block()
            .ok_or_else(|| "Failed to get latest block".to_string())?;

        let genesis_block = self
            .get_genesis_block()
            .ok_or_else(|| "Failed to get genesis block".to_string())?;

        Ok(Handshake {
            genesis_block_hash: genesis_block.hash,
            latest_block_hash: latest_block.hash,
            latest_block_index: latest_block.index,
        })
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
        NodeServices::start_services(config, self).await;
    }

    fn blockchain_write_to_file(&mut self) {
        match self.get_blocks() {
            Ok(blocks) => match serde_json::to_string_pretty(&blocks) {
                Ok(json_str) => {
                    let file_name = format!("{}_blockchain_blocks", &self.name);
                    if let Err(e) = write_to_file(&json_str, &file_name) {
                        error!("{}", e);
                    }
                }
                Err(e) => error!("Failed to serialize blocks: {}", e),
            },
            Err(e) => error!("Failed to retrieve blocks: {}", e),
        }

        match self.get_transactions_from_pool() {
            Ok(transactions) => match serde_json::to_string_pretty(&transactions) {
                Ok(json_str) => {
                    let file_name = format!("{}_tx_pool", &self.name);
                    if let Err(e) = write_to_file(&json_str, &file_name) {
                        error!("{}", e);
                    }
                }
                Err(e) => error!("Failed to serialize transactions: {}", e),
            },
            Err(e) => error!("Failed to retrieve transactions in transaction pool: {}", e),
        }
    }
}
