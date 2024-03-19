use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RideOffer{
    pub ride_request_transaction_hash:String,
    pub fare:u64,
}