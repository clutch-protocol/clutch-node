use std::result;

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
    pub fn verify_state(transaction: &Transaction, db: &Database) -> bool {
        true
    }

    pub fn state_transaction(
        transaction: &Transaction,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let ride_request: RideRequest = serde_json::from_str(&transaction.data.arguments).unwrap();
        let tx_hash = &transaction.hash;
        let ride_request_key = format!("ride_request_{}", tx_hash).into_bytes();
        let ride_request_value = serde_json::to_string(&ride_request).unwrap().into_bytes();

        vec![Some((ride_request_key, ride_request_value))]
    }

    pub fn get_ride_request(tx_hash: &str, db: &Database) -> Result<RideRequest, String> {
        let key = format!("ride_request_{}", tx_hash).into_bytes();
        match db.get("state", &key) {
            Ok(Some(value)) => {
                let account_state_str = match String::from_utf8(value) {
                    Ok(v) => v,
                    Err(_) => return Err("Failed to decode UTF-8 string".to_string()),
                };
                match serde_json::from_str(&account_state_str) {
                    Ok(ride_request) => Ok(ride_request),
                    Err(_) => Err("Failed to deserialize RideRequest".to_string()),
                }
            }
            Ok(None) => Err("No ride request found for the given transaction hash".to_string()),
            Err(_) => Err("Database error occurred".to_string()),
        }
    }
}
