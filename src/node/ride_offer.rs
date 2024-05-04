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

        let exsist_ride_request: bool =
            RideOffer::check_ride_request_exsist(&ride_request_tx_hash, &db);
        if !exsist_ride_request {
            println!("has_exsist_ride_request: {}", exsist_ride_request);
            return false;
        }

        let has_ride = RideOffer::check_has_ride(&ride_request_tx_hash, &db);
        if has_ride {
            println!("has_ride: {}", has_ride);
            return false;
        }

        return true;
    }

    fn check_ride_request_exsist(ride_request_tx_hash: &str, db: &Database) -> bool {
        match RideRequest::get_ride_request(&ride_request_tx_hash, db) {
            Ok(ride_request) => true,
            Err(_) => {
                println!(
                    "No ride request found for the given transaction hash: {}",
                    ride_request_tx_hash
                );
                false
            }
        }
    }

    fn check_has_ride(ride_request_tx_hash: &str, db: &Database) -> bool {
        match RideRequest::get_ride(&ride_request_tx_hash, &db) {
            Ok(None) => false,
            Ok(Some(ride_tx_hash)) => {
                println!("Ride request has a ride: {}", ride_tx_hash);
                true
            }
            Err(_) => {
                println!(
                    "No ride found for the given ride request transaction hash: {}",
                    ride_request_tx_hash
                );
                false
            }
        }
    }

    pub fn state_transaction(
        transaction: &Transaction,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let ride_offer: RideOffer = serde_json::from_str(&transaction.data.arguments).unwrap();
        let tx_hash = &transaction.hash;
        let ride_offer_key = Self::construct_ride_offer_key(tx_hash);
        let ride_offer_value = serde_json::to_string(&ride_offer).unwrap().into_bytes();

        vec![Some((ride_offer_key, ride_offer_value))]
    }

    pub fn get_ride_offer(ride_offer_tx_hash: &str, db: &Database) -> Result<RideOffer, String> {
        let key = Self::construct_ride_offer_key(ride_offer_tx_hash);
        match db.get("state", &key) {
            Ok(Some(value)) => {
                let account_state_str = match String::from_utf8(value) {
                    Ok(v) => v,
                    Err(_) => return Err("Failed to decode UTF-8 string".to_string()),
                };
                match serde_json::from_str(&account_state_str) {
                    Ok(ride_offer) => Ok(ride_offer),
                    Err(_) => Err("Failed to deserialize RideOffer".to_string()),
                }
            }
            Ok(None) => Err("No ride offer found for the given transaction hash".to_string()),
            Err(_) => Err("Database error occurred".to_string()),
        }
    }

    fn construct_ride_offer_key(ride_offer_tx_hash: &str) -> Vec<u8> {
        format!("ride_offer_{}", ride_offer_tx_hash).into_bytes()
    }

    pub fn construct_ride_offer_acceptance_key(
        ride_offer_tx_hash: &str,
        ride_tx_hash: &str,
    ) -> Vec<u8> {
        format!("ride_offer_{}:ride:{}", ride_offer_tx_hash, ride_tx_hash).into_bytes()
    }
}
