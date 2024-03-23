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
        let db =  rocks_db(&name);
        let blockchain = Blockchain {
            name: name,
            db:db,
        };

         // Serialize the genesis block and save to DB
         let genesis_block = Block::new_genesis_block();
         let serialized_block = serde_json::to_string(&genesis_block).unwrap();
         blockchain.db.put(b"block_0", serialized_block.as_bytes()).unwrap();

        let iter = blockchain.db.iterator(rocksdb::IteratorMode::Start); // From the start

        // Iterate through all key-value pairs in the database
        for (key, value) in iter {
            println!("{:?}: {:?}", key, value);
        }


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

pub fn rocks_db(name: &String) -> rocksdb::DBWithThreadMode<rocksdb::SingleThreaded> {
    let db_base_path = env::var("DB_PATH").expect("Failed to get DB_PATH env.");
    let db_path =  format!("{}/{}.db",db_base_path,name);

    let mut options = Options::default();
    options.create_if_missing(true);

    // Attempt to open the database with specified options
    let db = match DB::open(&options, &db_path) {
        Ok(db) => db,
        Err(e) => panic!("Failed to open database: {}", e),
    };
    
    db
}