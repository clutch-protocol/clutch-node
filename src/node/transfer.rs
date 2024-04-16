use crate::node::account_state::AccountState;
use serde::{Deserialize, Serialize};

use super::{database::Database, transaction::Transaction};

#[derive(Serialize, Deserialize)]
pub struct Transfer {
    pub to: String,
    pub value: f64,
}

impl Transfer {
    pub fn verify_state(transaction: &Transaction, from_account_state: &AccountState) -> bool {
        let transfer: Transfer = serde_json::from_str(&transaction.data.arguments).unwrap();
        if from_account_state.balance < transfer.value {
            println!(
                "Error: Insufficient balance.From:{} Required: {}, Available: {}",
                transaction.from, transfer.value, from_account_state.balance
            );

            return false;
        }

        true
    }

    pub fn state_transaction_transfer(
        transaction: &Transaction,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let transfer: Transfer = serde_json::from_str(&transaction.data.arguments).unwrap();
        let value = transfer.value;

        let from = &transaction.from;
        let mut from_account_state = AccountState::get_current_state(&from, &db);
        from_account_state.balance = from_account_state.balance - value;
        from_account_state.nonce = from_account_state.nonce + 1;
        let from_key = format!("account_state_{}", from).into_bytes();
        let from_serialized_balance = serde_json::to_string(&from_account_state)
            .unwrap()
            .into_bytes();

        let to = transfer.to;
        let mut to_account_state = AccountState::get_current_state(&to, &db);
        to_account_state.balance = to_account_state.balance + value;
        let to_key = format!("account_state_{}", to).into_bytes();
        let to_serialized_balance = serde_json::to_string(&to_account_state)
            .unwrap()
            .into_bytes();

        vec![
            Some((from_key, from_serialized_balance)),
            Some((to_key, to_serialized_balance)),
        ]
    }
}
