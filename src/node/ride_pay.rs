use crate::node::{
    account_state::AccountState, ride_acceptance::RideAcceptance, ride_offer::RideOffer,
};

use super::{database::Database, ride_request::RideRequest, transaction::Transaction};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RidePay {
    pub ride_acceptance_transaction_hash: String,
    pub fare: u64,
}

impl RidePay {
    pub fn verify_state(transaction: &Transaction, db: &Database) -> bool {
        let ride_pay = match serde_json::from_str::<RidePay>(&transaction.data.arguments) {
            Ok(ride_pay) => ride_pay,
            Err(_) => {
                println!("Failed to deserialize transaction data.");
                return false;
            }
        };

        let ride_acceptance_tx_hash = &ride_pay.ride_acceptance_transaction_hash;
        let ride_acceptance = match RideAcceptance::get_ride_acceptance(ride_acceptance_tx_hash, db)
        {
            Ok(Some(ride_acceptance)) => ride_acceptance,
            Ok(None) | Err(_) => {
                println!("Ride acceptance does not exist or failed to retrieve.");
                return false;
            }
        };

        let ride_offer =
            match RideOffer::get_ride_offer(&ride_acceptance.ride_offer_transaction_hash, db) {
                Ok(Some(ride_offer)) => ride_offer,
                Ok(None) | Err(_) => {
                    println!(
                        "Failed to retrieve ride offer '{}'.",
                        &ride_acceptance.ride_offer_transaction_hash
                    );
                    return false;
                }
            };

        let passenger = match RideRequest::get_from(&ride_offer.ride_request_transaction_hash, db) {
            Ok(Some(driver)) => driver,
            Ok(None) | Err(_) => {
                println!(
                    "Failed to retrieve 'from' field for ride offer with transaction hash '{}'.",
                    &ride_acceptance.ride_offer_transaction_hash
                );
                return false;
            }
        };

        if passenger.to_string() != transaction.from {
            println!(
                "Ride request 'from' field does not match the transaction 'from' field. Expected: {}, found: {}.",
                transaction.from, passenger
            );
            return false;
        }

        if ride_pay.fare > ride_offer.fare {
            println!(
                "The fare in the ride pay ({}) is greater than the fare in the ride offer ({}).",
                ride_pay.fare, ride_offer.fare
            );
            return false;
        }

        true
    }

    pub fn state_transaction(
        transaction: &Transaction,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let ride_pay: RidePay = match serde_json::from_str(&transaction.data.arguments) {
            Ok(ride_pay) => ride_pay,
            Err(_) => {
                eprintln!("Failed to deserialize transaction arguments.");
                return vec![];
            }
        };
        
        let ride_acceptance_tx_hash = &ride_pay.ride_acceptance_transaction_hash;

        let ride_pay_key = Self::construct_ride_pay_key(&transaction.hash);
        let ride_pay_value = serde_json::to_string(&ride_pay)
            .expect("Failed to serialize RidePay.")
            .into_bytes();

        let ride_pay_acceptance_key =
            Self::construct_ride_pay_acceptance_key(&ride_pay.ride_acceptance_transaction_hash);
        let ride_pay_acceptance_value = serde_json::to_string(&ride_acceptance_tx_hash)
            .unwrap()
            .into_bytes();

        let ride_acceptance = match RideAcceptance::get_ride_acceptance(ride_acceptance_tx_hash, db)
        {
            Ok(Some(ride_acceptance)) => ride_acceptance,
            Ok(None) | Err(_) => {
                eprintln!("Ride acceptance does not exist or failed to retrieve.");
                return vec![];
            }
        };

        let ride_offer_tx_hash = &ride_acceptance.ride_offer_transaction_hash;
        let driver = match RideOffer::get_from(ride_offer_tx_hash, db) {
            Ok(Some(driver)) => driver,
            Ok(None) | Err(_) => {
                eprintln!(
                    "Failed to retrieve 'from' field for ride offer with transaction hash '{}'.",
                    ride_offer_tx_hash
                );
                return vec![];
            }
        };

        let transfer_value: i64 = ride_pay.fare as i64;
        let (driver_account_state_key, driver_account_state_value) =
            AccountState::update_account_state_key(&driver, transfer_value, db);

        vec![
            Some((ride_pay_key, ride_pay_value)),
            Some((ride_pay_acceptance_key, ride_pay_acceptance_value)),
            Some((driver_account_state_key, driver_account_state_value)),
        ]
    }

    pub fn construct_ride_pay_key(tx_hash: &str) -> Vec<u8> {
        format!("ride_pay_{}", tx_hash).into_bytes()
    }

    pub fn construct_ride_pay_acceptance_key(tx_hash: &str) -> Vec<u8> {
        format!("ride_pay{}:ride_acceptance", tx_hash).into_bytes()
    }
}
