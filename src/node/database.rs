use rocksdb::{ColumnFamilyDescriptor, DBWithThreadMode, Options, SingleThreaded, WriteBatch, DB};
use std::env;

#[derive(Debug)]
pub struct Database {
    db: Option<DBWithThreadMode<SingleThreaded>>,
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
        let db_path = Database::db_path(&name);
        let mut options = Options::default();
        options.create_if_missing(true);
        options.create_missing_column_families(true);

        let db =
            DB::open_default(&db_path).expect("Failed to open database with default column family");

        Database { db: Some(db) }
    }
    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, String> {
        match &self.db {
            Some(db) => db.get(key).map_err(|e| e.to_string()),
            None => Err("Database connection is closed".to_string()),
        }
    }
    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), String> {
        match &self.db {
            Some(db) => db.put(key, value).map_err(|e| e.to_string()),
            None => Err("Database connection is closed".to_string()),
        }
    }

    pub fn write(&self, operations: Vec<(&[u8], &[u8])>) -> Result<(), String> {
        let mut batch = WriteBatch::default();

        for (key, value) in operations {
            batch.put(key, value); 
        }

        match &self.db {
            Some(db) => db.write(batch).map_err(|e| e.to_string()),
            None => Err("Database connection is closed".to_string()),
        }
    }

    pub fn close(&mut self) {
        let _ = self.db.take(); // Properly drops the database object, closing the connection
    }

    pub fn delete_database(&self, name: &str) -> Result<(), String> {
        let db_path = Database::db_path(&name);   
             
        DB::destroy(&Options::default(), db_path).map_err(|e| e.to_string())?;
        Ok(())
    }
}
