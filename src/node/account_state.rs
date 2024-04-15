use serde::{Deserialize, Serialize};
use crate::node::database::Database;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountState {
    public_key: String,
    balance: f64,
}

impl AccountState {
    pub fn new_account_state(public_key: &String, balance: f64) -> AccountState {
        AccountState {
            public_key: public_key.to_string(),
            balance: balance,
        }
    }

    pub fn get_current_state(public_key: &String ,db:&Database) -> f64 {
        let key = format!("balance_{}", public_key).into_bytes();
        match db.get(&key){
            Ok(Some(value)) => {
                let account_state_str = String::from_utf8(value).unwrap();                
                let account_state: AccountState = serde_json::from_str(&account_state_str).unwrap();
                account_state.balance
            },
            Ok(None) => 0.0 ,
            Err(_) => todo!(),
        }
    }
}
