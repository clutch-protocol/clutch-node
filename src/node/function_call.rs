use std::fmt;
use serde::{Deserialize,Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct FunctionCall {
    pub function_call_type: FunctionCallType,
    pub arguments: String,
}


#[derive(Debug,Serialize,Deserialize)]
pub enum FunctionCallType{
    RideRequest,
    RideOffer,
    RideAcceptance,
    ConfirmArrival,
    ComplainArrival,
    RidePayment,
}

impl fmt::Display for FunctionCallType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}