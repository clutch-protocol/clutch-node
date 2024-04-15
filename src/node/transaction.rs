use super::database::Database;
use super::signature_keys::{self, SignatureKeys};
use crate::node::account_state::AccountState;
use crate::node::function_call::{FunctionCall, FunctionCallType};
use crate::node::transfer::Transfer;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::vec;

const FROM_GENESIS: &str = "0xGENESIS";

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub data: FunctionCall,
    pub nonce: u64,
    pub signature_r: String,
    pub signature_s: String,
    pub signature_v: i32,
    pub hash: String,
}

impl Transaction {
    pub fn new_transaction<T: Serialize>(
        from: String,
        nonce: u64,
        function_call_type: FunctionCallType,
        secret_key: String,
        payload: T,
    ) -> Transaction {
        let arguments = serde_json::to_string(&payload).unwrap();
        let function_call = FunctionCall {
            function_call_type,
            arguments,
        };

        let mut transaction = Transaction {
            hash: String::new(),
            signature_r: String::new(),
            signature_s: String::new(),
            signature_v: 0,
            from: from,
            nonce: nonce,
            data: function_call,
        };

        transaction.hash = transaction.calculate_hash();
        let data = transaction.hash.as_bytes();
        let (r, s, v) = signature_keys::SignatureKeys::sign(&secret_key, data);
        transaction.signature_r = r;
        transaction.signature_s = s;
        transaction.signature_v = v;

        transaction
    }

    pub fn new_genesis_transactions() -> Vec<Transaction> {
        let from_secret_key = "d2c446110cfcecbdf05b2be528e72483de5b6f7ef9c7856df2f81f48e9f2748f";

        let tx1 = Self::new_transaction(
            FROM_GENESIS.to_string(),
            0,
            FunctionCallType::Transfer,
            from_secret_key.to_string(),
            Transfer {
                to: "0xdeb4cfb63db134698e1879ea24904df074726cc0".to_string(),
                value: 150.0,
            },
        );
        let tx2 = Self::new_transaction(
            FROM_GENESIS.to_string(),
            0,
            FunctionCallType::Transfer,
            from_secret_key.to_string(),
            Transfer {
                to: "0xa300e57228487edb1f5c0e737cbfc72d126b5bc2".to_string(),
                value: 90.0,
            },
        );
        let tx3 = Self::new_transaction(
            FROM_GENESIS.to_string(),
            0,
            FunctionCallType::Transfer,
            from_secret_key.to_string(),
            Transfer {
                to: "0xac20ff4e42ff243046faaf032068762dd2c018dc".to_string(),
                value: 80.0,
            },
        );
        let tx4 = Self::new_transaction(
            FROM_GENESIS.to_string(),
            0,
            FunctionCallType::Transfer,
            from_secret_key.to_string(),
            Transfer {
                to: "0xa91101310bee451ca0e219aba08d8d4dd929f16c".to_string(),
                value: 20.0,
            },
        );
        let tx5 = Self::new_transaction(
            FROM_GENESIS.to_string(),
            0,
            FunctionCallType::Transfer,
            from_secret_key.to_string(),
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
        if !self.verify_signature() {
            println!(
                "Verification failed: Signature does not match for transaction from {}",
                self.from
            );
            return false;
        }

        if !self.verify_nonce() {
            println!(
                "Verification failed: nonce does not match for transaction from {}",
                self.from
            );
            return false;
        }

        let is_valid_tx = match self.data.function_call_type {
            FunctionCallType::Transfer => {
                let transfer: Transfer = serde_json::from_str(&self.data.arguments).unwrap();
                let from = &self.from;
                let value = transfer.value;

                let from_balance_state = AccountState::get_current_state(&from, &db);
                if from_balance_state.balance < value {
                    println!(
                        "Error: Insufficient balance.From:{} Required: {}, Available: {}",
                        from, value, from_balance_state.balance
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

    fn verify_signature(&self) -> bool {
        let from_public_key = &self.from;
        let data = self.hash.as_bytes();
        let r = &self.signature_r;
        let s = &self.signature_s;
        let v = self.signature_v;

        SignatureKeys::verify(from_public_key, data, r, s, v)
    }
    
    fn verify_nonce(&self)-> bool {
        let nonce = self.nonce;

        true 
    }

    pub fn state_transaction(&self, db: &Database) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        match self.data.function_call_type {
            FunctionCallType::Transfer => {
                let transfer: Transfer = serde_json::from_str(&self.data.arguments).unwrap();
                let value = transfer.value;

                let from = &self.from;
                let mut from_account_state = AccountState::get_current_state(&from, &db);
                from_account_state.balance  = from_account_state.balance - value;
                let from_key = format!("balance_{}", from).into_bytes();
                let from_serialized_balance = serde_json::to_string(&from_account_state)
                    .unwrap()
                    .into_bytes();

                let to = transfer.to;
                let mut to_account_state = AccountState::get_current_state(&to, &db);
                to_account_state.balance = to_account_state.balance + value;                
                let to_key = format!("balance_{}", to).into_bytes();
                let to_serialized_balance = serde_json::to_string(&to_account_state)
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
