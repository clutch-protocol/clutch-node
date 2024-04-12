use rocksdb::{ColumnFamilyDescriptor, DBWithThreadMode, Options, SingleThreaded, WriteBatch, DB};
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

        // Check and open with the default column family
        let cf_descriptor = ColumnFamilyDescriptor::new("default", Options::default());
        let db = DB::open_cf_descriptors(&options, &db_path, vec![cf_descriptor])
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

    /// Deletes all data from the database.
    pub fn delete_all(&self) -> Result<(), String> {
        let start_key: &[u8; 0] = &[];
        let end_key: &[u8; 0] = &[];

        match self.db.cf_handle("default") {
            Some(cf_handle) => self
                .db
                .delete_range_cf(cf_handle, start_key, end_key)
                .map_err(|e| e.to_string()),
            None => Err("Default column family not found".to_string()),
        }
    }
}
