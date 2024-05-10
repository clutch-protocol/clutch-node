use crate::node::{account_state::AccountState, ride_request::RideRequest};
use serde::{Deserialize, Serialize};

use super::{database::Database, ride_offer::RideOffer, transaction::Transaction};

#[derive(Serialize, Deserialize)]
pub struct RideAcceptance {
    pub ride_offer_transaction_hash: String,
}

impl RideAcceptance {
    pub fn verify_state(transaction: &Transaction, db: &Database) -> bool {
        let ride_acceptanc: Result<RideAcceptance, _> =
            serde_json::from_str(&transaction.data.arguments);

        if ride_acceptanc.is_err() {
            println!("Failed to deserialize transaction data.");
            return false;
        }

        let ride_acceptanc = ride_acceptanc.unwrap();
        let ride_offer_transaction_hash = &ride_acceptanc.ride_offer_transaction_hash;

        if let Ok(Some(ride_offer)) = RideOffer::get_ride_offer(ride_offer_transaction_hash, db) {
            let fare = &ride_offer.fare;
            let from = &transaction.from;
            
            let passenger_account_state = AccountState::get_current_state(from, &db);
            if &passenger_account_state.balance < fare {
                println!(
                    "The account balance is insufficient to cover the fare for the requested ride. \
                     Account balance is: {}, fare: {}",
                    passenger_account_state.balance, fare
                );

                return false;
            }

            // Check if there is any ride linked to this ride offer's request.
            if let Ok(Some(_)) =
                RideRequest::get_ride(&ride_offer.ride_request_transaction_hash, db)
            {
                println!("A ride for the requested ride offer already exists.");
                return false;
            }

            // Check if this ride offer is already used in another ride.
            if let Ok(Some(_)) = RideOffer::get_ride(&ride_offer_transaction_hash, db) {
                println!("Ride offer is already linked to a ride.");
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
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let ride_acceptance: RideAcceptance =
            serde_json::from_str(&transaction.data.arguments).unwrap();

        let ride_offer_tx_hash = &ride_acceptance.ride_offer_transaction_hash;
        let ride_request_tx_hash = &RideOffer::get_ride_offer(&ride_offer_tx_hash, db)
            .unwrap()
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

        let ride_offer = RideOffer::get_ride_offer(&ride_offer_tx_hash, db)
            .unwrap()
            .unwrap();

        let transfer_value: i64 = ride_offer.fare as i64;
        let (passenger_account_state_key, passenger_account_state_value) =
            AccountState::update_account_state_key(&transaction.from, -transfer_value, db);

        vec![
            Some((ride_key, ride_value)),
            Some((ride_request_acceptance_key, ride_request_acceptance_value)),
            Some((ride_offer_acceptance_key, ride_offer_acceptance_value)),
            Some((passenger_account_state_key, passenger_account_state_value)),
        ]
    }

    pub fn construct_ride_key(tx_hash: &str) -> Vec<u8> {
        format!("ride_{}", tx_hash).into_bytes()
    }
}
