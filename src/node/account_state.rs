use crate::node::database::Database;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountState {
    pub public_key: String,
    pub balance: f64,
    pub nonce: u64,
}

impl AccountState {
    fn new_account_state(public_key: &String) -> AccountState {
        AccountState {
            public_key: public_key.to_string(),
            balance: 0.0,
            nonce: 0,
        }
    }

    pub fn get_current_state(public_key: &String, db: &Database) -> AccountState {
        let key = format!("account_state_{}", public_key).into_bytes();
        match db.get(&key) {
            Ok(Some(value)) => {
                let account_state_str = String::from_utf8(value).unwrap();
                let account_state: AccountState = serde_json::from_str(&account_state_str).unwrap();
                account_state
            }
            Ok(None) => Self::new_account_state(public_key),
            Err(_) => todo!(),
        }
    }
}
