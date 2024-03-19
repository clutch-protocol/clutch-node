use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct ComplainArrival{
    confirm_arrival_transaction_hash:String,
}