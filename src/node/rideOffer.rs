use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RideOffer{
    pub rideRequest_transaction_hash:String,
    pub fare:u64,
}