use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}
