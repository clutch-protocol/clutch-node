use crate::node::account_state::AccountState;
use crate::node::database::Database;
use crate::node::transaction::Transaction;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Transfer {
    pub to: String,
    pub value: f64,
}

impl Transfer {
    pub fn verify_state(transaction: &Transaction, db: &Database) -> bool {
        let from = &transaction.from;
        let transfer: Transfer = serde_json::from_str(&transaction.data.arguments).unwrap();
        let from_account_state = AccountState::get_current_state(from, &db);
        if from_account_state.balance < transfer.value {
            println!(
                "Error: Insufficient balance.From:{} Required: {}, Available: {}",
                transaction.from, transfer.value, from_account_state.balance
            );

            return false;
        }

        true
    }

    pub fn state_transaction(
        transaction: &Transaction,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let transfer: Transfer = serde_json::from_str(&transaction.data.arguments).unwrap();
        let transfer_value = transfer.value;

        let from = &transaction.from;
        let (from_key, from_serialized_state) =
            AccountState::update_account_state_key(from, -transfer_value, db);

        let to = &transfer.to;
        let (to_key, to_serialized_state) = AccountState::update_account_state_key(to, transfer_value, db);

        vec![
            Some((from_key, from_serialized_state)),
            Some((to_key, to_serialized_state)),
        ]
    }
}
