use rocksdb::{DBWithThreadMode, Options, SingleThreaded, DB};
use std::env;

#[derive(Debug)]
pub struct Database {
    db: DBWithThreadMode<SingleThreaded>,
}

impl Database {
    pub fn new_db(name: &str) -> Self {
        let db_base_path = env::var("DB_PATH").unwrap_or_else(|_| {
            let current_dir = env::current_dir().expect("Failed to get current directory");
            current_dir.to_str().unwrap_or(".").to_string()
        });

        let db_path = format!("{}/{}.db", db_base_path, name);

        let mut options = Options::default();
        options.create_if_missing(true);

        let db = DB::open(&options, db_path).expect("Failed to open database");
        Database { db }
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, String>  {
        self.db.get(key).map_err(|e| e.to_string())
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), String> {
        self.db.put(key, value).map_err(|e| e.to_string())
    }
}
