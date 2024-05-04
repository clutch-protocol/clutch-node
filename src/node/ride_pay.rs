use super::{database::Database, ride_offer::RideOffer, transaction::Transaction};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RidePay {
    pub ride_transaction_hash: String,
    pub fare: u64,
}

impl RidePay {
    pub fn verify_state(transaction: &Transaction, db: &Database) -> bool {
        true
    }

    pub fn state_transaction(
        transaction: &Transaction,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        vec![]
    }
}
