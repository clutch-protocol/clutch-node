use crate::node::{
    account_state::AccountState, ride_acceptance::RideAcceptance, ride_offer::RideOffer,
};

use super::{database::Database, ride_request::RideRequest, transaction::Transaction};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RideCancel {
    pub ride_acceptance_transaction_hash: String,
}

impl RideCancel {
    pub fn verify_state(transaction: &Transaction, db: &Database) -> bool {
        let ride_cancel = match serde_json::from_str::<RideCancel>(&transaction.data.arguments) {
            Ok(ride_cancel) => ride_cancel,
            Err(_) => {
                println!("Failed to deserialize transaction data.");
                return false;
            }
        };

        let ride_acceptance_tx_hash = &ride_cancel.ride_acceptance_transaction_hash;
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
                    "Failed to retrieve 'from' field for ride request with transaction hash '{}'.",
                    &ride_offer.ride_request_transaction_hash
                );
                return false;
            }
        };

        let driver = match RideOffer::get_from(&ride_acceptance.ride_offer_transaction_hash, db) {
            Ok(Some(driver)) => driver,
            Ok(None) | Err(_) => {
                println!(
                    "Failed to retrieve 'from' field for ride offer with transaction hash '{}'.",
                    &ride_acceptance.ride_offer_transaction_hash
                );
                return false;
            }
        };

        if passenger.to_string() != transaction.from && driver.to_string() != transaction.from {
            println!(
                "Transaction 'from' field does not match the expected values. Expected either passenger: '{}' or driver: '{}', but found: '{}'.",
                passenger, driver, transaction.from
            );
            return false;
        }

        true
    }

    pub fn state_transaction(
        transaction: &Transaction,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let ride_cancel: RideCancel = match serde_json::from_str(&transaction.data.arguments) {
            Ok(ride_pay) => ride_pay,
            Err(_) => {
                eprintln!("Failed to deserialize transaction arguments.");
                return vec![];
            }
        };

        let ride_cancel_key = Self::construct_ride_cancel_key(&transaction.hash);
        let ride_cancel_value = serde_json::to_string(&ride_cancel)
            .expect("Failed to serialize RidePay.")
            .into_bytes();

        let ride_acceptance_tx_hash = &ride_cancel.ride_acceptance_transaction_hash;

        let ride_acceptance_cancel_key =
            RideAcceptance::construct_ride_acceptance_cancel_key(&ride_acceptance_tx_hash);
        let ride_acceptance_cancel_value = serde_json::to_string(&transaction.hash)
            .unwrap()
            .into_bytes();

        vec![
            Some((ride_cancel_key, ride_cancel_value)),
            Some((ride_acceptance_cancel_key, ride_acceptance_cancel_value)),
        ]
    }

    pub fn construct_ride_cancel_key(tx_hash: &str) -> Vec<u8> {
        format!("ride_pay_{}", tx_hash).into_bytes()
    }
}
