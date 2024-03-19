use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct RideAcceptance{
    pub ride_request_transaction_hash: String,
    pub ride_offer_transaction_hash : String,
}