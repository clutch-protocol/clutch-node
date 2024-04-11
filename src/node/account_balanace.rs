use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalance {
    public_key: String,
    balance: f64,
}

impl AccountBalance {
    pub fn new_account_balance(public_key: String, balance: f64) -> AccountBalance {
        AccountBalance {
            public_key: public_key,
            balance: balance,
        }
    }
}
