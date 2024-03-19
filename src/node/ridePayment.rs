use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct RidePayment{
    pub ride_acceptance_transaction_hash:String,
    pub fare: u64,
}