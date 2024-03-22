use rocksdb::{DBPath, Options, DB};
use std::env;
use crate::node::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub name: String,   
    db: DB,
}

impl Blockchain {
    pub fn new(name:String) -> Blockchain {        

        let db_base_path = env::var("DB_PATH").expect("Failed to get DB_PATH env.");        
        let db_path =  format!("{}/{}_database.db",db_base_path,name);

        let mut options = Options::default();
        options.create_if_missing(true); 

        // Attempt to open the database with specified options
        let db = match DB::open(&options, &db_path) {
            Ok(db) => db,
            Err(e) => panic!("Failed to open database: {}", e),
        };

        let blockchain = Blockchain {
            name: name,      
            db: db,
        };

         // Serialize the genesis block and save to DB
         let genesis_block = Block::new_genesis_block();  
         let serialized_block = serde_json::to_string(&genesis_block).unwrap();
         blockchain.db.put(b"genesis", serialized_block.as_bytes()).unwrap();
 
         blockchain        
    }

    pub fn block_import(&mut self, block:Block){    

        // for tx in block.transactions {
        //     match tx.data.function_call_type {
        //         _ => {},
        //     }
        // }   
         
        // self.blocks.push(block);        
    }
}