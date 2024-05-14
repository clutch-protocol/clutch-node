use crate::node::{
    account_state::AccountState, ride_acceptance::RideAcceptance, ride_offer::RideOffer,
};

use super::{database::Database, transaction::Transaction};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RidePay {
    pub ride_acceptance_transaction_hash: String,
    pub fare: u64,
}

impl RidePay {
    pub fn verify_state(transaction: &Transaction, db: &Database) -> bool {
        let ride_pay: Result<RideAcceptance, _> = serde_json::from_str(&transaction.data.arguments);

        if ride_pay.is_err() {
            println!("Failed to deserialize transaction data.");
            return false;
        }

        let ride_pay = ride_pay.unwrap();
        let ride_acceptance_tx_hash = &ride_pay.ride_offer_transaction_hash;

        if let Ok(Some(ride_acceptance)) =
            RideAcceptance::get_ride_acceptance(&ride_acceptance_tx_hash, &db)
        {
            if let Ok(Some(driver)) =
                RideOffer::get_from(&ride_acceptance.ride_offer_transaction_hash, &db)
            {
                if &driver.to_string() != &transaction.from {
                    println!("Ride offer 'from' field does not match the transaction 'from' field. Expected: {}, found: {}.", transaction.from, driver);
                    return false;
                }
            } else {
                println!(
                    "Failed to retrieve 'from' field for ride offer with transaction hash '{}'.",
                    &ride_acceptance.ride_offer_transaction_hash
                );
                return false;
            }
        } else {
            println!("Ride acceptance does not exist or failed to retrieve.");
            return false;
        }

        true
    }

    pub fn state_transaction(
        transaction: &Transaction,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let ride_pay: RidePay = serde_json::from_str(&transaction.data.arguments).unwrap();
        let ride_pay_key = Self::construct_ride_pay_key(&transaction.hash);
        let ride_pay_value = serde_json::to_string(&ride_pay).unwrap().into_bytes();

        let ride_acceptance_tx_hash = ride_pay.ride_acceptance_transaction_hash;
        let ride_acceptance = RideAcceptance::get_ride_acceptance(&ride_acceptance_tx_hash, &db);
        let ride_offer_tx_hash = ride_acceptance
            .unwrap()
            .unwrap()
            .ride_offer_transaction_hash;

        let ride_offer = RideOffer::get_ride_offer(&ride_offer_tx_hash, &db);
        let ride_fare = ride_offer.unwrap().unwrap().fare;
        let driver = RideOffer::get_from(&ride_offer_tx_hash, &db)
            .unwrap()
            .unwrap();

        let transfer_value: i64 = ride_fare as i64;
        let (driver_account_state_key, driver_account_state_value) =
            AccountState::update_account_state_key(&driver, transfer_value, db);

        vec![
            Some((ride_pay_key, ride_pay_value)),
            Some((driver_account_state_key, driver_account_state_value)),
        ]
    }

    pub fn construct_ride_pay_key(tx_hash: &str) -> Vec<u8> {
        format!("ride_pay{}", tx_hash).into_bytes()
    }
}
