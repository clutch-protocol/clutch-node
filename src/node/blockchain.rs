use crate::node::account_balanace::AccountBalance;
use crate::node::block::Block;
use crate::node::database::Database;
use crate::node::function_call::FunctionCallType;
use crate::node::transaction::Transaction;
use crate::node::transfer::Transfer;

#[derive(Debug)]
pub struct Blockchain {
    pub name: String,
    db: Database,
    latest_block_index: usize,
}

impl Blockchain {
    pub fn new(name: String) -> Blockchain {
        let db = Database::new_db(&name);
        let mut blockchain = Blockchain {
            name: name,
            db: db,
            latest_block_index: 0,
        };

        blockchain.latest_block_index = Blockchain::get_latest_block_index(&blockchain);
        blockchain.genesis_block_import();
        blockchain
    }

    pub fn get_latest_block_index(&self) -> usize {
        match self.db.get(b"block_latest_block_index") {
            Ok(Some(value)) => {
                let index_str = String::from_utf8(value).unwrap();
                index_str.parse::<usize>().unwrap()
            }
            Ok(None) => 0,
            Err(_) => panic!("Failed to retrieve the latest block index"),
        }
    }

    pub fn block_import(&mut self, block: Block) {
        let is_valid_block = block.validate_block(self);
        if !is_valid_block {
            println!("Block is invalid and will not be added.");
            return;
        }

        for tx in block.transactions.iter() {
            let is_valid_tx = tx.validate_transaction();
            if !is_valid_tx {
                println!("Block contains invalid transactions and will not be added.");
                return;
            }
        }

        self.add_block_to_chain(block);
    }

    fn genesis_block_import(&mut self) {
        match self.db.get(b"block_0") {
            Ok(Some(_)) => {
                println!("Genesis block already exists.");
            }
            Ok(None) => {
                println!("Genesis block does not exist, creating new one...");
                let genesis_block = Block::new_genesis_block();
                self.add_block_to_chain(genesis_block);
            }
            Err(e) => panic!("Failed to check for genesis block: {}", e),
        }
    }

    fn add_block_to_chain(&mut self, block: Block) {
        let mut keys: Vec<Vec<u8>> = Vec::new();
        let mut values: Vec<Vec<u8>> = Vec::new();

        //State block
        match block.state_block() {
            Some((block_keys, block_values)) => {
                keys.extend(block_keys);
                values.extend(block_values);
            }
            None => {
                println!("Failed to serialize block for storage.");
                return;
            }
        };

        //State transactions
        for tx in block.transactions.iter() {
            match tx.state_transaction(&self.db) {
                updates  => {
                    for update in updates {
                        for (key, value) in update {
                            keys.push(key);
                            values.push(value);
                        }
                    }
                },
                _=> println!("Error processing transaction states")
            }
        }

        //Map operations
        let mut operations: Vec<(&[u8], &[u8])> = Vec::new();
        for (key, value) in keys.iter().zip(values.iter()) {
            operations.push((key, value));
        }

        //Update Database
        match self.db.write(operations) {
            Ok(_) => {
                println!(
                    "add_block_to_chain successfully. block index:{}",
                    block.index
                );

                self.latest_block_index = block.index;
            }
            Err(e) => panic!("Failed add_block_to_chain: {}", e),
        }
    }
}
