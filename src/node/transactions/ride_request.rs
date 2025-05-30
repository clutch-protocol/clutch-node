use crate::node::account_state::AccountState;
use crate::node::coordinate::Coordinates;
use crate::node::database::Database;

use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RideRequest {
    pub pickup_location: Coordinates,
    pub dropoff_location: Coordinates,
    pub fare: u64,
}

impl RideRequest {
    pub fn verify_state(&self, from: &String, db: &Database) -> Result<(), String> {
        let passenger_account_state = AccountState::get_current_state(from, &db);

        if passenger_account_state.balance < self.fare {
            return Err(format!(
                "The account balance is insufficient to cover the fare for the requested ride. \
                 Account balance is: {}, fare: {}",
                passenger_account_state.balance, self.fare
            ));
        }

        Ok(())
    }

    pub fn state_transaction(
        &self,
        from: &String,
        tx_hash :&String,
        _db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {

        let ride_request_key = Self::construct_ride_request_key(tx_hash);
        let ride_request_value = serde_json::to_string(&self).unwrap().into_bytes();

        let ride_request_from_key = Self::construct_ride_request_from_key(&tx_hash);
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

    pub fn get_ride_acceptance(
        ride_request_tx_hash: &str,
        db: &Database,
    ) -> Result<Option<String>, String> {
        let key = Self::construct_ride_request_acceptance_key(ride_request_tx_hash);
        match db.get("state", &key) {
            Ok(Some(value)) => match String::from_utf8(value) {
                Ok(ride_tx_has) => Ok(Some(ride_tx_has)),
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
                Ok(from) => Ok(Some(from)),
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
        format!("ride_request_{}:ride_acceptance", ride_request_tx_hash).into_bytes()
    }
}

impl Encodable for RideRequest {
    fn rlp_append(&self, stream: &mut RlpStream) {
        // Begin an RLP list with three elements: pickup_location, dropoff_location, and fare
        stream.begin_list(3);
        stream.append(&self.pickup_location);
        stream.append(&self.dropoff_location);
        stream.append(&self.fare);
    }
}

impl Decodable for RideRequest {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        if !rlp.is_list() || rlp.item_count()? != 3 {
            return Err(DecoderError::RlpIncorrectListLen);
        }
        
        Ok(RideRequest {
            pickup_location: rlp.val_at(0)?,
            dropoff_location: rlp.val_at(1)?,
            fare: rlp.val_at(2)?,
        })
    }
}
