use crate::node::database::Database;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountState {
    pub public_key: String,
    pub balance: u64,
}

impl AccountState {
    fn new_account_state(public_key: &str) -> AccountState {
        AccountState {
            public_key: public_key.to_string(),
            balance: 0,
        }
    }

    pub fn get_current_state(public_key: &String, db: &Database) -> AccountState {
        let key = Self::construct_account_state_key(public_key);
        match db.get("state", &key) {
            Ok(Some(value)) => {
                let account_state_str = String::from_utf8(value).unwrap();
                let account_state: AccountState = serde_json::from_str(&account_state_str).unwrap();
                account_state
            }
            Ok(None) => Self::new_account_state(public_key),
            Err(_) => todo!(),
        }
    }

    fn construct_account_state_key(public_key: &str) -> Vec<u8> {
        format!("account_state_{}", public_key).into_bytes()
    }

    pub fn update_account_state_key(
        public_key: &String,
        balance_change: i64,
        db: &Database,
    ) -> (Vec<u8>, Vec<u8>) {
        let mut from_account_state = AccountState::get_current_state(&public_key, &db);
        from_account_state.balance = (from_account_state.balance as i64 + balance_change) as u64;

        let from_key = Self::construct_account_state_key(public_key);
        let from_serialized_balance = serde_json::to_string(&from_account_state)
            .unwrap()
            .into_bytes();
        (from_key, from_serialized_balance)
    }

    pub fn get_current_nonce(public_key: &String, db: &Database) -> u64 {
        let key = Self::construct_account_nonce_key(public_key);
        match db.get("state", &key) {
            Ok(Some(value)) => {
                let bytes_array: [u8; 8] = value.try_into().expect("Slice with incorrect length");
                u64::from_be_bytes(bytes_array)
            }
            Ok(None) => 0, // No value found, returning default nonce
            Err(_) => todo!(),
        }
    }

    fn construct_account_nonce_key(public_key: &str) -> Vec<u8> {
        format!("account_nonce_{}", public_key).into_bytes()
    }

    pub fn increase_account_nonce_key(public_key: &String, db: &Database) -> (Vec<u8>, Vec<u8>) {
        let nonce = AccountState::get_current_nonce(public_key, db) + 1;
        let account_nonce_key = Self::construct_account_nonce_key(public_key);
        let account_nonce_serlized = nonce.to_be_bytes().to_vec();
        (account_nonce_key, account_nonce_serlized)
    }
}
