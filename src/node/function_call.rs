use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    pub function_call_type: FunctionCallType,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FunctionCallType {
    Transfer,
    RideRequest,
    RideOffer,
    RideAcceptance,
    RidePay,
    RideCancel,
    ConfirmArrival,
    ComplainArrival,
}

impl fmt::Display for FunctionCallType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
