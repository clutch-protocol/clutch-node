use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct ConfirmArrival{
    pub ride_acceptance_transaction_hash:String,
}