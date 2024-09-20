
use crate::node::{
    account_state::AccountState, database::Database, function_call::{FunctionCall, FunctionCallType}, signature_keys::{self, SignatureKeys}, transactions::{ride_acceptance::RideAcceptance, ride_cancel::RideCancel, ride_offer::RideOffer, ride_pay::RidePay, ride_request::RideRequest}
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::vec;

use super::transfer::Transfer;


const FROM_GENESIS: &str = "0xGENESIS";

#[derive(Debug, Serialize, Deserialize,Clone)]
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
        transaction
    }

    pub fn new_genesis_transactions() -> Vec<Transaction> {
        let tx1 = Self::new_transaction(
            FROM_GENESIS.to_string(),
            0,
            FunctionCallType::Transfer,
            Transfer {
                to: "0xdeb4cfb63db134698e1879ea24904df074726cc0".to_string(),
                value: 30,
            },
        );
        let tx2 = Self::new_transaction(
            FROM_GENESIS.to_string(),
            0,
            FunctionCallType::Transfer,
            Transfer {
                to: "0xa300e57228487edb1f5c0e737cbfc72d126b5bc2".to_string(),
                value: 90,
            },
        );
        let tx3 = Self::new_transaction(
            FROM_GENESIS.to_string(),
            0,
            FunctionCallType::Transfer,
            Transfer {
                to: "0xac20ff4e42ff243046faaf032068762dd2c018dc".to_string(),
                value: 80,
            },
        );
        let tx4 = Self::new_transaction(
            FROM_GENESIS.to_string(),
            0,
            FunctionCallType::Transfer,
            Transfer {
                to: "0xa91101310bee451ca0e219aba08d8d4dd929f16c".to_string(),
                value: 20,
            },
        );
        let tx5 = Self::new_transaction(
            FROM_GENESIS.to_string(),
            0,
            FunctionCallType::Transfer,
            Transfer {
                to: "0x37adf81cb1f18762042e5da03a55f1e54ba66870".to_string(),
                value: 45,
            },
        );

        vec![tx1, tx2, tx3, tx4, tx5]
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}",
            self.from, self.data.function_call_type, self.data.arguments, self.nonce,
        ));
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    #[allow(dead_code)]
    pub fn sign(&mut self, secret_key: &str) {
        let hash_bytes = self.hash.as_bytes();
        let (r, s, v) = signature_keys::SignatureKeys::sign(secret_key, hash_bytes);

        self.signature_r = r;
        self.signature_s = s;
        self.signature_v = v;
    }

    fn verify_signature(&self) -> Result<bool, String> {
        let from_public_key = &self.from;
        let data = self.hash.as_bytes();
        let r = &self.signature_r;
        let s = &self.signature_s;
        let v = self.signature_v;

        SignatureKeys::verify(from_public_key, data, r, s, v)
    }

    pub fn validate_transactions(
        db: &Database,
        transactions: &Vec<Transaction>,
    ) -> Result<(), String> {
        if transactions.is_empty() {
            return Err("No transactions to validate.".to_string());
        }

        for tx in transactions.iter() {
            tx.validate_transaction(&db)?;
        }

        Ok(())
    }

    pub fn validate_transaction(&self, db: &Database) -> Result<(), String> {
        self.verify_signature()?;
        self.verify_nonce(db)?;
        self.verify_state(db)?;

        Ok(())
    }

    fn verify_nonce(&self, db: &Database) -> Result<bool, String> {
        match AccountState::get_current_nonce(&self.from, db) {
            Ok(last_nonce) => {
                let nonce = self.nonce;
                if nonce != last_nonce + 1 {
                    return Err(format!(
                        "Verification failed: Incorrect nonce for transaction from '{}'. Expected: {}, got: {}.",
                        self.from, last_nonce + 1, nonce
                    ));
                }
                Ok(true)
            }
            Err(e) => Err(format!(
                "Verification failed: Unable to retrieve nonce for transaction from '{}'. Error: {}",
                self.from, e
            )),
        }
    }

    fn verify_state(&self, db: &Database) -> Result<(), String> {
        return match self.data.function_call_type {
            FunctionCallType::Transfer => Transfer::verify_state(&self, db),
            FunctionCallType::RideRequest => RideRequest::verify_state(&self, db),
            FunctionCallType::RideOffer => RideOffer::verify_state(&self, db),
            FunctionCallType::RideAcceptance => RideAcceptance::verify_state(&self, db),
            FunctionCallType::RidePay => RidePay::verify_state(&self, db),
            FunctionCallType::RideCancel => RideCancel::verify_state(&self, db),
            _ => Err("Unknown function call type.".to_string()),
        };
    }

    pub fn state_transaction(&self, db: &Database) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let states = match self.data.function_call_type {
            FunctionCallType::Transfer => Transfer::state_transaction(&self, &db),
            FunctionCallType::RideRequest => RideRequest::state_transaction(&self, db),
            FunctionCallType::RideOffer => RideOffer::state_transaction(&self, db),
            FunctionCallType::RideAcceptance => RideAcceptance::state_transaction(&self, db),
            FunctionCallType::RidePay => RidePay::state_transaction(&self, db),
            FunctionCallType::RideCancel => RideCancel::state_transaction(&self, db),
            _ => vec![None],
        };

        let (nonce_key, nonce_serialized) =
            AccountState::increase_account_nonce_key(&self.from, db).unwrap();

        let mut results = states;
        results.push(Some((nonce_key, nonce_serialized)));
        results
    }
}
