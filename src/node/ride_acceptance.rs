use serde::{Deserialize, Serialize};

use crate::node::{ride_acceptance, ride_request::RideRequest};

use super::{database::Database, ride_offer::RideOffer, transaction::Transaction};

#[derive(Serialize, Deserialize)]
pub struct RideAcceptance {
    pub ride_offer_transaction_hash: String,
}

impl RideAcceptance {
    pub fn verify_state(transaction: &Transaction, db: &Database) -> bool {
        let ride_acceptanc: RideAcceptance =
            serde_json::from_str(&transaction.data.arguments).unwrap();
        let ride_offer_transaction_hash = &ride_acceptanc.ride_offer_transaction_hash;

        let exist_ride_offer =
            RideAcceptance::check_exist_ride_offer(&ride_offer_transaction_hash, &db);
        if !exist_ride_offer {
            println!("has_exsist_ride_offer: {}", exist_ride_offer);
            return false;
        }

        let has_ride =
            RideAcceptance::check_has_ride_by_ride_offer(&ride_offer_transaction_hash, &db);
        if has_ride {
            println!("has_ride: {}", has_ride);
            return false;
        }

        return true;
    }

    fn check_exist_ride_offer(ride_offer_transaction_hash: &str, db: &Database) -> bool {
        match RideOffer::get_ride_offer(&ride_offer_transaction_hash, db) {
            Ok(ride_offer) => {
                return true;
            }
            Err(e) => {
                println!(
                    "No ride offer found for the given transaction hash: {},ex:{}",
                    ride_offer_transaction_hash, e
                );
                return false;
            }
        }
    }

    fn check_has_ride_by_ride_offer(ride_offer_transaction_hash: &str, db: &Database) -> bool {
        match RideOffer::get_ride(&ride_offer_transaction_hash, &db) {
            Ok(None) => false,
            Ok(Some(ride_tx_hash)) => {
                println!("Ride Offer has a ride: {}", ride_tx_hash);
                true
            }
            Err(_) => {
                println!(
                    "No ride found for the given ride Offer transaction hash: {}",
                    ride_offer_transaction_hash
                );
                false
            }
        }
    }

    pub fn state_transaction(
        transaction: &Transaction,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let ride_acceptance: RideAcceptance =
            serde_json::from_str(&transaction.data.arguments).unwrap();

        let ride_offer_tx_hash = &ride_acceptance.ride_offer_transaction_hash;
        let ride_request_tx_hash = &RideOffer::get_ride_offer(&ride_offer_tx_hash, db)
            .unwrap()
            .ride_request_transaction_hash;

        let ride_tx_hash = &transaction.hash;
        let ride_key = Self::construct_ride_key(&ride_tx_hash);
        let ride_value = serde_json::to_string(&ride_acceptance)
            .unwrap()
            .into_bytes();

        let ride_request_acceptance_key =
            RideRequest::construct_ride_request_acceptance_key(&ride_request_tx_hash);
        let ride_request_acceptance_value =
            serde_json::to_string(&ride_tx_hash).unwrap().into_bytes();

        let ride_offer_acceptance_key =
            RideOffer::construct_ride_offer_acceptance_key(&ride_offer_tx_hash);
        let ride_offer_acceptance_value =
            serde_json::to_string(&ride_tx_hash).unwrap().into_bytes();

        vec![
            Some((ride_key, ride_value)),
            Some((ride_request_acceptance_key, ride_request_acceptance_value)),
            Some((ride_offer_acceptance_key, ride_offer_acceptance_value)),
        ]
    }

    fn construct_ride_key(tx_hash: &str) -> Vec<u8> {
        format!("ride_{}", tx_hash).into_bytes()
    }
}
