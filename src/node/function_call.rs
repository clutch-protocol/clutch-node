use std::fmt;

#[derive(Debug)]
pub struct FunctionCall {
    pub function_call_type: FunctionCallType,
    pub arguments: String,
}


#[derive(Debug)]
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