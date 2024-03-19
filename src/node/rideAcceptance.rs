use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct RideAcceptance{
    pub rideRequest_transaction_hash: String,
    pub rideOffer_transaction_hash : String,
}