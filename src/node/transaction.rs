use crate::node::account_balanace::AccountBalance;
use crate::node::complain_arrival::ComplainArrival;
use crate::node::confirm_arrival::ConfirmArrival;
use crate::node::function_call::{FunctionCall, FunctionCallType};
use crate::node::ride_acceptance::RideAcceptance;
use crate::node::ride_offer::RideOffer;
use crate::node::ride_payment::RidePayment;
use crate::node::ride_request::RideRequest;
use crate::node::signature_keys::SignatureKeys;
use crate::node::transfer::Transfer;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::vec;

use super::database::Database;
use super::signature_keys;

const FROM_GENESIS: &str = "0xGENESIS";

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub data: FunctionCall,
    pub nonce: u64,
    // pub signature_r: String,
    // pub signature_s: String,
    // pub signature_v: i32,
    pub hash: String,
}

impl Transaction {
    pub fn new_transaction<T: Serialize>(
        from: String,
        nonce: u64,
        function_call_type: FunctionCallType,
        payload: T,
    ) -> Transaction {
        let arguments = serde_json::to_string(&payload).unwrap();
        let function_call = FunctionCall {
            function_call_type,
            arguments,
        };

        let mut transaction = Transaction {
            hash: String::new(),
            from: from,
            nonce: nonce,
            data: function_call,
        };

        transaction.hash = transaction.calculate_hash();
        transaction
    }

    pub fn new_genesis_transactions() -> Vec<Transaction> {
        let tx1 = Self::new_transaction(
            FROM_GENESIS.to_string(),
            0,
            FunctionCallType::Transfer,
            Transfer {
                to: "0xb87a9ac289f679f1f489fefa14f885187e311e2f".to_string(),
                value: 100.0,
            },
        );
        let tx2 = Self::new_transaction(
            FROM_GENESIS.to_string(),
            0,
            FunctionCallType::Transfer,
            Transfer {
                to: "0xa300e57228487edb1f5c0e737cbfc72d126b5bc2".to_string(),
                value: 90.0,
            },
        );
        let tx3 = Self::new_transaction(
            FROM_GENESIS.to_string(),
            0,
            FunctionCallType::Transfer,
            Transfer {
                to: "0xac20ff4e42ff243046faaf032068762dd2c018dc".to_string(),
                value: 80.0,
            },
        );
        let tx4 = Self::new_transaction(
            FROM_GENESIS.to_string(),
            0,
            FunctionCallType::Transfer,
            Transfer {
                to: "0xa91101310bee451ca0e219aba08d8d4dd929f16c".to_string(),
                value: 20.0,
            },
        );
        let tx5 = Self::new_transaction(
            FROM_GENESIS.to_string(),
            0,
            FunctionCallType::Transfer,
            Transfer {
                to: "0x37adf81cb1f18762042e5da03a55f1e54ba66870".to_string(),
                value: 45.0,
            },
        );

        vec![tx1, tx2, tx3, tx4, tx5]
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}",
            self.from, self.data.function_call_type, self.data.arguments
        ));
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn validate_transaction(&self, db: &Database) -> bool {
        // e.g., check sender's balance, verify digital signature, etc.
        //signature_keys::SignatureKeys::verify(&self, data, signature)

        let is_valid_tx = match self.data.function_call_type {
            FunctionCallType::Transfer => {
                let transfer: Transfer = serde_json::from_str(&self.data.arguments).unwrap();
                let from = &self.from;
                let value = transfer.value;

                let from_balance = AccountBalance::get_current_balance(&from, &db);
                if from_balance < value {
                    println!(
                        "Error: Insufficient balance.From:{} Required: {}, Available: {}",
                        from, value, from_balance
                    );
                    return false;
                }

                true
            }
            FunctionCallType::RideRequest => {
                // Validation logic for RideRequest
                true
            }
            FunctionCallType::RideOffer => {
                // Validation logic for RideOffer
                true
            }
            FunctionCallType::RideAcceptance => {
                // Validation logic for RideAcceptance
                true
            }
            FunctionCallType::ConfirmArrival => {
                // Validation logic for ConfirmArrival
                true
            }
            FunctionCallType::ComplainArrival => {
                // Validation logic for ComplainArrival
                true
            }
            FunctionCallType::RidePayment => {
                // Validation logic for RidePayment
                // This might include checking the ride status, confirming the fare, etc.
                true
            }
            _ => false, // Add more types as needed
        };

        // If all transactions are valid, return true
        is_valid_tx
    }

    pub fn verify_signature(&self) -> bool {
        let from_public_key = &self.from;
        //let data = self.hash;

        // SignatureKeys::verify(from_public_key, data, signature)
        true
    }

    pub fn state_transaction(&self, db: &Database) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        match self.data.function_call_type {
            FunctionCallType::Transfer => {
                let transfer: Transfer = serde_json::from_str(&self.data.arguments).unwrap();
                let value = transfer.value;

                let from = &self.from;
                let from_balance = AccountBalance::get_current_balance(&from, &db) - value;
                let from_account_balance = AccountBalance::new_account_balance(&from, from_balance);
                let from_key = format!("balance_{}", from).into_bytes();
                let from_serialized_balance = serde_json::to_string(&from_account_balance)
                    .unwrap()
                    .into_bytes();

                let to = transfer.to;
                let to_balance = AccountBalance::get_current_balance(&to, &db) + value;
                let to_account_balance = AccountBalance::new_account_balance(&to, to_balance);
                let to_key = format!("balance_{}", to).into_bytes();
                let to_serialized_balance = serde_json::to_string(&to_account_balance)
                    .unwrap()
                    .into_bytes();

                vec![
                    Some((from_key, from_serialized_balance)),
                    Some((to_key, to_serialized_balance)),
                ]
            }
            _ => vec![None],
        }
    }
}
