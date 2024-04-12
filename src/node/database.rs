use rocksdb::{ColumnFamilyDescriptor, DBWithThreadMode, Options, SingleThreaded, WriteBatch, DB};
use std::env;

#[derive(Debug)]
pub struct Database {
    db: DBWithThreadMode<SingleThreaded>,
}

impl Database {

    fn db_path(name: &str) -> String {
        let db_base_path = env::var("DB_PATH").unwrap_or_else(|_| {
            let current_dir = env::current_dir().expect("Failed to get current directory");
            current_dir.to_str().unwrap_or(".").to_string()
        });
        format!("{}/{}.db", db_base_path, name)
    }
    
    pub fn new_db(name: &str) -> Self {       
        let db_path =Database::db_path(&name);
        let mut options = Options::default();
        options.create_if_missing(true);
        options.create_missing_column_families(true);
        
        let db = DB::open_default(&db_path)
            .expect("Failed to open database with default column family");

        Database { db }
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, String> {
        self.db.get(key).map_err(|e| e.to_string())
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), String> {
        self.db.put(key, value).map_err(|e| e.to_string())
    }

    pub fn write(&self, operations: Vec<(&[u8], &[u8])>) -> Result<(), String> {
        let mut batch = WriteBatch::default();

        // Iterate over the operations and add them to the batch
        for (key, value) in operations {
            batch.put(key, value); // No error handling here, as put on a WriteBatch does not fail
        }

        // Perform the batch write
        self.db.write(batch).map_err(|e| e.to_string())
    }

    pub fn delete_database(&self, name: &str) -> Result<(), String> {        
        let db_path =Database::db_path(&name);                     
        DB::destroy(&Options::default(), db_path).map_err(|e| e.to_string())?;
        Ok(())
    }
}
