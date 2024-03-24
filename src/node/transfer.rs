use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Transfer {
    pub to: String,
    pub value: f64, 
}