use crate::node::account_state::AccountState;
use crate::node::database::Database;
use crate::node::transaction::Transaction;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Transfer {
    pub to: String,
    pub value: u64,
}

impl Transfer {   
    pub fn verify_state(transaction: &Transaction, db: &Database) -> Result<(), String> {
        let from = &transaction.from;
        
        let transfer: Transfer = serde_json::from_str(&transaction.data.arguments)
            .map_err(|e| format!("Failed to parse transfer data: {}", e))?;
        
        let from_account_state = AccountState::get_current_state(from, &db);
        
        if from_account_state.balance < transfer.value {
            return Err(format!(
                "Error: Insufficient balance. From: {} Required: {}, Available: {}",
                transaction.from, transfer.value, from_account_state.balance
            ));
        }
    
        Ok(())
    }

    pub fn state_transaction(
        transaction: &Transaction,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let transfer: Transfer = serde_json::from_str(&transaction.data.arguments).unwrap();
        let transfer_value: i64 = transfer.value as i64;

        let (from_account_state_key, from_account_state_value) =
            AccountState::update_account_state_key(&transaction.from, -transfer_value, db);

        let to = &transfer.to;
        let (to_account_state_key, to_account_state_value) =
            AccountState::update_account_state_key(to, transfer_value, db);

        vec![
            Some((from_account_state_key, from_account_state_value)),
            Some((to_account_state_key, to_account_state_value)),
        ]
    }
}
