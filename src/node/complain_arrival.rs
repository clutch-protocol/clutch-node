use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct ComplainArrival{
    pub ride_acceptance_transaction_hash:String,
}