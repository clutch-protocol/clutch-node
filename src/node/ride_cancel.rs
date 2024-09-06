use crate::node::{
    account_state::AccountState, ride_acceptance::RideAcceptance, ride_offer::RideOffer,
};

use super::{database::Database, ride_request::RideRequest, transaction::Transaction};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Serialize, Deserialize)]
pub struct RideCancel {
    pub ride_acceptance_transaction_hash: String,
}

impl RideCancel {
    pub fn verify_state(transaction: &Transaction, db: &Database) -> Result<(), String> {
        let ride_cancel = serde_json::from_str::<RideCancel>(&transaction.data.arguments)
            .map_err(|_| "Failed to deserialize transaction data.".to_string())?;
    
        let ride_acceptance_tx_hash = &ride_cancel.ride_acceptance_transaction_hash;
        let ride_acceptance = RideAcceptance::get_ride_acceptance(ride_acceptance_tx_hash, db)
            .map_err(|_| "Ride acceptance does not exist or failed to retrieve.".to_string())?
            .ok_or_else(|| "Ride acceptance does not exist.".to_string())?;
    
        let ride_cancel_exists = match RideAcceptance::get_ride_cancel(ride_acceptance_tx_hash, db) {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(_) => {
                return Err(format!(
                    "Failed to retrieve ride cancel for transaction hash '{}'.",
                    ride_acceptance_tx_hash
                ));
            }
        };
    
        if ride_cancel_exists {
            return Err("A ride cancel for the requested ride acceptance already exists.".to_string());
        }
    
        let ride_offer = RideOffer::get_ride_offer(&ride_acceptance.ride_offer_transaction_hash, db)
            .map_err(|_| format!(
                "Failed to retrieve ride offer '{}'.",
                &ride_acceptance.ride_offer_transaction_hash
            ))?
            .ok_or_else(|| "Ride offer does not exist.".to_string())?;
    
        let passenger = RideRequest::get_from(&ride_offer.ride_request_transaction_hash, db)
            .map_err(|_| format!(
                "Failed to retrieve 'from' field for ride request with transaction hash '{}'.",
                &ride_offer.ride_request_transaction_hash
            ))?
            .ok_or_else(|| "Ride request does not exist.".to_string())?;
    
        let driver = RideOffer::get_from(&ride_acceptance.ride_offer_transaction_hash, db)
            .map_err(|_| format!(
                "Failed to retrieve 'from' field for ride offer with transaction hash '{}'.",
                &ride_acceptance.ride_offer_transaction_hash
            ))?
            .ok_or_else(|| "Ride offer does not exist.".to_string())?;
    
        let fare_paid = RideAcceptance::get_fare_paid(ride_acceptance_tx_hash, db)
            .map_err(|_| format!(
                "Failed to retrieve 'fare_paid' field for ride acceptance with transaction hash '{}'.",
                ride_acceptance_tx_hash
            ))?
            .unwrap_or(0);
    
        if (fare_paid as u64) == ride_offer.fare {
            return Err(format!(
                "The full fare for ride acceptance '{}' has been paid. No further payments are needed, and the ride cannot be cancelled.",
                ride_acceptance_tx_hash
            ));
        }
    
        if passenger.to_string() != transaction.from && driver.to_string() != transaction.from {
            return Err(format!(
                "Transaction 'from' field does not match the expected values. Expected either passenger: '{}' or driver: '{}', but found: '{}'.",
                passenger, driver, transaction.from
            ));
        }
    
        Ok(())
    }    

    pub fn state_transaction(
        transaction: &Transaction,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let ride_cancel: RideCancel = match serde_json::from_str(&transaction.data.arguments) {
            Ok(ride_pay) => ride_pay,
            Err(_) => {
                error!("Failed to deserialize transaction arguments.");
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

        let fare_paid = match RideAcceptance::get_fare_paid(&ride_acceptance_tx_hash, db) {
            Ok(Some(fare)) => fare,
            Ok(None) => 0,
            Err(_) => {
                error!(
                        "Failed to retrieve 'fare_paid' field for ride acceptace with transaction hash '{}'.",
                        &ride_acceptance_tx_hash
                    );
                0
            }
        };

        let ride_acceptance = RideAcceptance::get_ride_acceptance(ride_acceptance_tx_hash, db)
            .unwrap()
            .unwrap();

        let ride_offer =
            RideOffer::get_ride_offer(&ride_acceptance.ride_offer_transaction_hash, db)
                .unwrap()
                .unwrap();

        let passenger = RideRequest::get_from(&ride_offer.ride_request_transaction_hash, db)
            .unwrap()
            .unwrap();

        let remaining_amount = (ride_offer.fare as i64) - (fare_paid as i64);

        let (passenger_account_state_key, passenger_account_state_value) =
            AccountState::update_account_state_key(&passenger, remaining_amount, db);

        vec![
            Some((ride_cancel_key, ride_cancel_value)),
            Some((passenger_account_state_key, passenger_account_state_value)),
            Some((ride_acceptance_cancel_key, ride_acceptance_cancel_value)),
        ]
    }

    pub fn construct_ride_cancel_key(tx_hash: &str) -> Vec<u8> {
        format!("ride_pay_{}", tx_hash).into_bytes()
    }
}
