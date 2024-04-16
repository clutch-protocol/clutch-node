use crate::node::account_state::AccountState;
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
    pub fn verify_state(transaction: &Transaction, from_account_state: &AccountState) -> bool {
        true
    }

    pub fn state_transaction(
        transaction: &Transaction,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let ride_request: RideRequest = serde_json::from_str(&transaction.data.arguments).unwrap();
        let from = &transaction.from;
        let ride_request_key = format!("ride_request_{}", from).into_bytes();
        let ride_request_value = serde_json::to_string(&ride_request).unwrap().into_bytes();

        vec![Some((ride_request_key, ride_request_value))]
    }
}
