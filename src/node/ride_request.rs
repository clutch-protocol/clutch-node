use crate::node::coordinate::Coordinates;
use crate::node::database::Database;
use crate::node::transaction::Transaction;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RideRequest {
    pub pickup_location: Coordinates,
    pub dropoff_location: Coordinates,
}

impl RideRequest {
    pub fn verify_state(_transaction: &Transaction, _db: &Database) -> bool {
        true
    }

    pub fn state_transaction(
        transaction: &Transaction,
        _db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let ride_request: RideRequest = serde_json::from_str(&transaction.data.arguments).unwrap();
        let ride_request_tx_hash = &transaction.hash;
        let from = &transaction.from;

        let ride_request_key = Self::construct_ride_request_key(ride_request_tx_hash);
        let ride_request_value = serde_json::to_string(&ride_request).unwrap().into_bytes();

        let ride_request_from_key = Self::construct_ride_request_from_key(&ride_request_tx_hash);
        let ride_request_from_value = from.clone().into_bytes();

        vec![
            Some((ride_request_key, ride_request_value)),
            Some((ride_request_from_key, ride_request_from_value)),
        ]
    }

    pub fn get_ride_request(
        ride_request_tx_hash: &str,
        db: &Database,
    ) -> Result<Option<RideRequest>, String> {
        let key = Self::construct_ride_request_key(ride_request_tx_hash);
        match db.get("state", &key) {
            Ok(Some(value)) => {
                let ride_request_str = match String::from_utf8(value) {
                    Ok(v) => v,
                    Err(_) => return Err("Failed to decode UTF-8 string".to_string()),
                };
                match serde_json::from_str(&ride_request_str) {
                    Ok(ride_request) => Ok(ride_request),
                    Err(_) => Err("Failed to deserialize RideRequest".to_string()),
                }
            }
            Ok(None) => Ok(None),
            Err(_) => Err("Database error occurred".to_string()),
        }
    }

    pub fn get_ride(ride_request_tx_hash: &str, db: &Database) -> Result<Option<String>, String> {
        let key = Self::construct_ride_request_acceptance_key(ride_request_tx_hash);
        match db.get("state", &key) {
            Ok(Some(value)) => match String::from_utf8(value) {
                Ok(v) => Ok(Some(v)),
                Err(_) => return Err("Failed to decode UTF-8 string".to_string()),
            },
            Ok(None) => Ok(None),
            Err(_) => Err("Database error occurred".to_string()),
        }
    }

    pub fn get_from(ride_request_tx_hash: &str, db: &Database) -> Result<Option<String>, String> {
        let key = Self::construct_ride_request_from_key(ride_request_tx_hash);
        match db.get("state", &key) {
            Ok(Some(value)) => match String::from_utf8(value) {
                Ok(v) => Ok(Some(v)),
                Err(_) => return Err("Failed to decode UTF-8 string".to_string()),
            },
            Ok(None) => Ok(None),
            Err(_) => Err("Database error occurred".to_string()),
        }
    }

    fn construct_ride_request_key(ride_request_tx_hash: &str) -> Vec<u8> {
        format!("ride_request_{}", ride_request_tx_hash).into_bytes()
    }

    pub fn construct_ride_request_from_key(ride_request_tx_hash: &str) -> Vec<u8> {
        format!("ride_request_{}:from", ride_request_tx_hash).into_bytes()
    }

    pub fn construct_ride_request_acceptance_key(ride_request_tx_hash: &str) -> Vec<u8> {
        format!("ride_request_{}:ride", ride_request_tx_hash).into_bytes()
    }
}
