use super::{database::Database, transaction::Transaction};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RidePay {
    pub ride_acceptance_transaction_hash: String,
    pub fare: u64,
}

impl RidePay {
    pub fn verify_state(_transaction: &Transaction, _db: &Database) -> bool {
        true
    }

    pub fn state_transaction(
        transaction: &Transaction,
        _db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let ride_pay: RidePay = serde_json::from_str(&transaction.data.arguments).unwrap();
        let ride_acceptance_tx_hash = ride_pay.ride_acceptance_transaction_hash;

        vec![]
    }

    pub fn construct_ride_pay_key(tx_hash: &str) -> Vec<u8> {
        format!("ride_{}", tx_hash).into_bytes()
    }
}
