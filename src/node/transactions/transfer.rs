use crate::node::account_state::AccountState;
use crate::node::database::Database;

use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transfer {
    pub to: String,
    pub value: u64,
}

impl Transfer {
    pub fn verify_state(&self, from: &String, db: &Database) -> Result<(), String> {
        let from_account_state = AccountState::get_current_state(from, db);

        if from_account_state.balance < self.value {
            return Err(format!(
                "Error: Insufficient balance. From: {} Required: {}, Available: {}",
                from, self.value, from_account_state.balance
            ));
        }

        Ok(())
    }

    pub fn state_transaction(
        &self,
        from: &String,
        db: &Database,
    ) -> Vec<Option<(Vec<u8>, Vec<u8>)>> {
        let transfer_value: i64 = self.value as i64;

        // Update sender's account state by deducting the transfer value
        let (from_account_state_key, from_account_state_value) =
            AccountState::update_account_state_key(from, -transfer_value, db);

        // Update recipient's account state by adding the transfer value
        let (to_account_state_key, to_account_state_value) =
            AccountState::update_account_state_key(&self.to, transfer_value, db);

        vec![
            Some((from_account_state_key, from_account_state_value)),
            Some((to_account_state_key, to_account_state_value)),
        ]
    }
}

impl Encodable for Transfer {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream.begin_list(2);
        stream.append(&self.to);
        stream.append(&self.value);
    }
}

impl Decodable for Transfer {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(Transfer {
            to: rlp.val_at(0)?,
            value: rlp.val_at(1)?,
        })
    }
}
