use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};
use crate::node::database::Database;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalance {
    public_key: String,
    balance: f64,
}

impl AccountBalance {
    pub fn new_account_balance(public_key: &String, balance: f64) -> AccountBalance {
        AccountBalance {
            public_key: public_key.to_string(),
            balance: balance,
        }
    }

    pub fn get_current_balance(public_Key: &String ,db:&Database) -> f64 {
        let key = format!("balance_{}", public_Key).into_bytes();
        match db.get(&key){
            Ok(Some(value)) => {
                let index_str = String::from_utf8(value).unwrap();
                index_str.parse::<f64>().unwrap()
            },
            Ok(None) => 0.0 ,
            Err(_) => todo!(),
        }
    }
}
