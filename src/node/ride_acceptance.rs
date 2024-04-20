use serde::{Deserialize, Serialize};

use super::{database::Database, ride_offer::RideOffer, transaction::Transaction};

#[derive(Serialize, Deserialize)]
pub struct RideAcceptance {
    pub ride_request_transaction_hash: String,
    pub ride_offer_transaction_hash: String,
}

impl RideAcceptance {
    pub fn verify_state(transaction: &Transaction, db: &Database) -> bool {
        let ride_acceptanc: RideAcceptance =
            serde_json::from_str(&transaction.data.arguments).unwrap();
        let ride_offer_tx_hash = ride_acceptanc.ride_offer_transaction_hash;
        match RideOffer::get_ride_offer(&ride_offer_tx_hash, db) {
            Ok(ride_offer) => {
                return true;
            }
            Err(_) => {
                println!(
                    "No ride offer found for the given transaction hash: {}",
                    ride_offer_tx_hash
                );
                return false;
            }
        }
    }

    pub fn state_transaction(
        transaction: &Transaction,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let ride_acceptance: RideAcceptance =
            serde_json::from_str(&transaction.data.arguments).unwrap();
        let tx_hash = &transaction.hash;
        let ride_acceptance_key = format!("ride_acceptance_{}", tx_hash).into_bytes();
        let ride_acceptance_value = serde_json::to_string(&ride_acceptance)
            .unwrap()
            .into_bytes();

        vec![Some((ride_acceptance_key, ride_acceptance_value))]
    }
}
