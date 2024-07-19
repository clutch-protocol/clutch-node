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
    pub fn verify_state(transaction: &Transaction, db: &Database) -> Result<(), String> {
        let ride_pay = serde_json::from_str::<RidePay>(&transaction.data.arguments)
            .map_err(|_| "Failed to deserialize transaction data.".to_string())?;
    
        let ride_acceptance_tx_hash = &ride_pay.ride_acceptance_transaction_hash;
        let ride_acceptance = RideAcceptance::get_ride_acceptance(ride_acceptance_tx_hash, db)
            .map_err(|_| "Ride acceptance does not exist or failed to retrieve.".to_string())?
            .ok_or("Ride acceptance does not exist.")?;
    
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
            .ok_or("Ride offer does not exist.")?;
    
        let passenger = RideRequest::get_from(&ride_offer.ride_request_transaction_hash, db)
            .map_err(|_| format!(
                "Failed to retrieve 'from' field for ride request with transaction hash '{}'.",
                &ride_offer.ride_request_transaction_hash
            ))?
            .ok_or("Ride request does not exist.")?;
    
        let fare_paid = RideAcceptance::get_fare_paid(ride_acceptance_tx_hash, db)
            .map_err(|_| format!(
                "Failed to retrieve 'fare_paid' field for ride acceptance with transaction hash '{}'.",
                &ride_acceptance_tx_hash
            ))?
            .unwrap_or(0);
    
        if passenger.to_string() != transaction.from {
            return Err(format!(
                "Ride request 'from' field does not match the transaction 'from' field. Expected: {}, found: {}.",
                transaction.from, passenger
            ));
        }
    
        let total_fare = (fare_paid as u64) + ride_pay.fare;
        if total_fare > ride_offer.fare {
            return Err(format!(
                "The total fare in the ride pay ({}) is greater than the fare in the ride offer ({}).",
                total_fare, ride_offer.fare
            ));
        }
    
        Ok(())
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

        let ride_acceptance = RideAcceptance::get_ride_acceptance(ride_acceptance_tx_hash, db)
            .unwrap()
            .unwrap();

        let ride_offer_tx_hash = &ride_acceptance.ride_offer_transaction_hash;
        let driver = RideOffer::get_from(ride_offer_tx_hash, db)
            .unwrap()
            .unwrap();

        let transfer_value: i64 = ride_pay.fare as i64;
        let (driver_account_state_key, driver_account_state_value) =
            AccountState::update_account_state_key(&driver, transfer_value, db);

        let fare_paid = match RideAcceptance::get_fare_paid(&ride_acceptance_tx_hash, db) {
            Ok(Some(fare)) => fare,
            Ok(None) => 0,
            Err(_) => {
                println!(
                    "Failed to retrieve 'fare_paid' field for ride acceptace with transaction hash '{}'.",
                    &ride_acceptance_tx_hash
                );
                0
            }
        };

        let total_fare = (fare_paid as u64) + ride_pay.fare;
        let fare_paid_key =
            RideAcceptance::construct_ride_acceptance_fare_paid_key(&ride_acceptance_tx_hash);
        let fare_paid_value = serde_json::to_string(&total_fare).unwrap().into_bytes();

        vec![
            Some((ride_pay_key, ride_pay_value)),
            Some((driver_account_state_key, driver_account_state_value)),
            Some((fare_paid_key, fare_paid_value)),
        ]
    }

    pub fn construct_ride_pay_key(tx_hash: &str) -> Vec<u8> {
        format!("ride_pay_{}", tx_hash).into_bytes()
    }
}
