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

    pub fn get_transactions(db: &Database) -> Result<Vec<Transaction>, String> {
        match db.get_keys_values_by_cf_name("tx_pool") {
            Ok(entries) => {
                let mut transactions = Vec::new();

                for (_key, value) in entries {
                    match serde_json::from_slice::<Transaction>(&value) {
                        Ok(transaction) => {
                            transactions.push(transaction);
                        }
                        Err(e) => {
                            return Err(format!("Failed to deserialize transaction: {}", e));
                        }
                    }
                }

                Ok(transactions)
            }
            Err(e) => Err(format!("Failed to retrieve transactions: {}", e)),
        }
    }
}
