use crate::node::coordinate::Coordinates;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RideRequest {
    pub pickup_location: Coordinates,
    pub dropoff_location: Coordinates,
}