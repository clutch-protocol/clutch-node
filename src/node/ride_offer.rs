use super::ride_request::RideRequest;
use crate::node::{database::Database, transaction::Transaction};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RideOffer {
    pub ride_request_transaction_hash: String,
    pub fare: u64,
}

impl RideOffer {
    pub fn verify_state(transaction: &Transaction, db: &Database) -> bool {
        let ride_offer: RideOffer = serde_json::from_str(&transaction.data.arguments).unwrap();
        let ride_request_tx_hash = ride_offer.ride_request_transaction_hash;

        if let Ok(Some(_)) = RideRequest::get_ride_request(&ride_request_tx_hash, db) {
            // Check if there is any ride linked to this ride offer's request.
            if let Ok(Some(_)) = RideRequest::get_ride(&ride_request_tx_hash, db) {
                println!("A ride for the requested ride offer already exists.");
                return false;
            }
        } else {
            println!("Ride offer does not exist or failed to retrieve.");
            return false;
        }

        true
    }

    pub fn state_transaction(
        transaction: &Transaction,
        _db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let ride_offer: RideOffer = serde_json::from_str(&transaction.data.arguments).unwrap();
        let ride_offer_tx_hash = &transaction.hash;
        let from = &transaction.from;

        let ride_offer_key = Self::construct_ride_offer_key(ride_offer_tx_hash);
        let ride_offer_value = serde_json::to_string(&ride_offer).unwrap().into_bytes();

        let ride_offer_from_key = Self::construct_ride_offer_from_key(&ride_offer_tx_hash);
        let ride_offer_from_value = from.clone().into_bytes();

        vec![
            Some((ride_offer_key, ride_offer_value)),
            Some((ride_offer_from_key, ride_offer_from_value)),
        ]
    }

    pub fn get_ride_offer(
        ride_offer_tx_hash: &str,
        db: &Database,
    ) -> Result<Option<RideOffer>, String> {
        let key = Self::construct_ride_offer_key(ride_offer_tx_hash);
        match db.get("state", &key) {
            Ok(Some(value)) => {
                let ride_offer_str = match String::from_utf8(value) {
                    Ok(v) => v,
                    Err(_) => return Err("Failed to decode UTF-8 string".to_string()),
                };
                match serde_json::from_str(&ride_offer_str) {
                    Ok(ride_offer) => Ok(ride_offer),
                    Err(_) => Err("Failed to deserialize RideOffer".to_string()),
                }
            }
            Ok(None) => Ok(None),
            Err(_) => Err("Database error occurred".to_string()),
        }
    }

    pub fn get_ride(ride_offer_tx_hash: &str, db: &Database) -> Result<Option<String>, String> {
        let key = Self::construct_ride_offer_acceptance_key(ride_offer_tx_hash);
        match db.get("state", &key) {
            Ok(Some(value)) => match String::from_utf8(value) {
                Ok(v) => Ok(Some(v)),
                Err(_) => return Err("Failed to decode UTF-8 string".to_string()),
            },
            Ok(None) => {
                // println!(" No data found.{}", &ride_request_tx_hash);
                Ok(None)
            }
            Err(_) => Err("Database error occurred".to_string()),
        }
    }

    pub fn get_from(ride_offer_tx_hash: &str, db: &Database) -> Result<Option<String>, String> {
        let key = Self::construct_ride_offer_from_key(ride_offer_tx_hash);
        match db.get("state", &key) {
            Ok(Some(value)) => match String::from_utf8(value) {
                Ok(from) => Ok(Some(from)),
                Err(_) => return Err("Failed to decode UTF-8 string".to_string()),
            },
            Ok(None) => Ok(None),
            Err(_) => Err("Database error occurred".to_string()),
        }
    }

    fn construct_ride_offer_key(ride_offer_tx_hash: &str) -> Vec<u8> {
        format!("ride_offer_{}", ride_offer_tx_hash).into_bytes()
    }

    pub fn construct_ride_offer_acceptance_key(ride_offer_tx_hash: &str) -> Vec<u8> {
        format!("ride_offer_{}:ride", ride_offer_tx_hash).into_bytes()
    }

    pub fn construct_ride_offer_from_key(ride_request_tx_hash: &str) -> Vec<u8> {
        format!("ride_offer_{}:from", ride_request_tx_hash).into_bytes()
    }
}
