use crate::node::coordinate::Coordinates;

pub struct RideRequest {
    pub pickup_location: Coordinates,
    pub dropoff_location: Coordinates,
}