use std::env;
use rocksdb::{Options, DB};

pub struct Database{

}

impl Database {
    pub fn new_db(name: &str) -> rocksdb::DBWithThreadMode<rocksdb::SingleThreaded> {
        let db_base_path = env::var("DB_PATH").unwrap_or_else(|_| {
            let current_dir = env::current_dir().expect("Failed to get current directory");
            current_dir.to_str().unwrap_or(".").to_string()
        });

        let db_path = format!("{}/{}.db", db_base_path, name);

        let mut options = Options::default();
        options.create_if_missing(true);

        match DB::open(&options, &db_path) {
            Ok(db) => db,
            Err(e) => panic!("Failed to open database: {}", e),
        }
    }
}
