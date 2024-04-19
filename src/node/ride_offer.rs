use serde::{de::value, Deserialize, Serialize};

use crate::node::{account_state::AccountState, database::Database, transaction::Transaction};

use super::ride_request::{self, RideRequest};

#[derive(Serialize, Deserialize)]
pub struct RideOffer {
    pub ride_request_transaction_hash: String,
    pub fare: u64,
}

impl RideOffer {
    pub fn verify_state(transaction: &Transaction, db: &Database) -> bool {
        let ride_offer: RideOffer = serde_json::from_str(&transaction.data.arguments).unwrap();
        let ride_request_tx_hash = ride_offer.ride_request_transaction_hash;
        match RideRequest::get_ride_request(&ride_request_tx_hash, db) {
            Ok(ride_request) => {
                return true;
            }
            Err(_) => {
                println!(
                    "No ride request found for the given transaction hash: {}",
                    ride_request_tx_hash
                );
                return false;
            }
        }
    }

    pub fn state_transaction(
        transaction: &Transaction,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let ride_offer: RideOffer = serde_json::from_str(&transaction.data.arguments).unwrap();
        let tx_hash = &transaction.hash;
        let ride_offer_key = format!("ride_offer_{}", tx_hash).into_bytes();
        let ride_offer_value = serde_json::to_string(&ride_offer).unwrap().into_bytes();

        vec![Some((ride_offer_key, ride_offer_value))]
    }
}
