use crate::node::database::Database;
use crate::node::transaction::Transaction;

pub struct TransactionPool {}

impl TransactionPool {
    pub fn add_transaction(&self, db: &Database, transaction: Transaction) {
        let value = serde_json::to_string(&transaction).unwrap().into_bytes();
        let key = Self::construct_tx_pool_key(&transaction.hash);

        db.put("tx_pool", &key, &value).unwrap();
    }

    pub fn construct_tx_pool_key(tx_hash: &str) -> Vec<u8> {
        format!("tx_pool_{}", tx_hash).into_bytes()
    }
}
