use serde::{Deserialize, Serialize};

use crate::node::{account_state::AccountState, database::Database, transaction::Transaction};

#[derive(Serialize, Deserialize)]
pub struct RideOffer {
    pub ride_request_transaction_hash: String,
    pub fare: u64,
}

impl RideOffer {
    pub fn verify_state(transaction: &Transaction, db: &Database) -> bool {
        let ride_offer: RideOffer = serde_json::from_str(&transaction.data.arguments).unwrap();
        let ride_request_tx_hash = ride_offer.ride_request_transaction_hash;
        
        true
    }

    pub fn state_transaction(
        transaction: &Transaction,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        vec![]
    }
}
