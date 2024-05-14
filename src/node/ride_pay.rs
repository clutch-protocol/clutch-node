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
    pub fn verify_state(_transaction: &Transaction, _db: &Database) -> bool {
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
        let driver = RideOffer::get_from(&ride_offer_tx_hash, &db).unwrap().unwrap();

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
