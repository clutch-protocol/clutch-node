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
}

impl Blockchain {
    pub fn new(name: String) -> Blockchain {
        let db = Database::new_db(&name);
        let mut blockchain = Blockchain { name: name, db: db };

        blockchain.genesis_block_import();
        blockchain
    }

    fn genesis_block_import(&mut self) {
        match self.db.get(b"block_0") {
            Ok(Some(_)) => {
                println!("Genesis block already exists.");
            }
            Ok(None) => {
                println!("Genesis block does not exist, creating new one...");
                let genesis_block = Block::new_genesis_block();                

                let mut keys: Vec<Vec<u8>> = Vec::new();
                let mut values: Vec<Vec<u8>> = Vec::new();

                for tx in genesis_block.transactions.iter() {
                    match tx.data.function_call_type {
                        FunctionCallType::Transfer => {
                            let transfer: Transfer =
                                serde_json::from_str(&tx.data.arguments).unwrap();

                            let account_balance = AccountBalance::new_account_balance(
                                transfer.to.to_string(),
                                transfer.value,
                            );

                            let key = format!("balance_{}", transfer.to).into_bytes();
                            let serialized_balance = serde_json::to_string(&account_balance)
                                .unwrap()
                                .into_bytes();

                            keys.push(key);
                            values.push(serialized_balance);
                        }
                        FunctionCallType::RideRequest => todo!(),
                        FunctionCallType::RideOffer => todo!(),
                        FunctionCallType::RideAcceptance => todo!(),
                        FunctionCallType::ConfirmArrival => todo!(),
                        FunctionCallType::ComplainArrival => todo!(),
                        FunctionCallType::RidePayment => todo!(),
                    }
                }

                let mut operations: Vec<(&[u8], &[u8])> = Vec::new();
                
                let serialized_block = serde_json::to_string(&genesis_block).unwrap();
                operations.push((b"block_0", serialized_block.as_bytes()));

                for (key, value) in keys.iter().zip(values.iter()) {
                    operations.push((key, value));
                }

                match self.db.write(operations) {
                    Ok(_) => println!("Genesis block and account balances stored successfully."),
                    Err(e) => panic!("Failed to store genesis block and account balances: {}", e),
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
