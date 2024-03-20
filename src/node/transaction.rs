use sha2::{Sha256, Digest};
use crate::node::ride_request::RideRequest;
use crate::node::ride_offer::RideOffer;
use crate::node::ride_acceptance::RideAcceptance;
use crate::node::confirm_arrival::ConfirmArrival;
use crate::node::complain_arrival::ComplainArrival;
use crate::node::ride_payment::RidePayment;

pub struct Transaction {
    pub from: String,
    pub to: Option<String>,
    pub value: Option<f64>, 
    pub hash: String,
    pub data: FunctionCall,
}

pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}


impl Transaction{

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", self.from, self.data.name, self.data.arguments));
        let result = hasher.finalize();
        format!("{:x}", result)                 
    }

    pub fn new_genesis_transactions() -> Vec<Transaction> {
        vec![]
    }

    fn new_tranaction(from: String, function_call: FunctionCall) -> Transaction {
        let mut transaction = Transaction {         
                hash: String::new(),   
                from:from,
                to: None,
                value: None,
                data: function_call,
            };
    
        transaction.hash = transaction.calculate_hash();
        transaction
    }

    pub fn new_ride_request_tranaction(from: String, ride_request: RideRequest) -> Transaction {
        let function_call = FunctionCall {
            name: "rideRequest".to_string(),
            arguments: serde_json::to_string(&ride_request).unwrap()
        };

        Transaction::new_tranaction(from, function_call)
    }

    pub fn new_ride_offer_tranaction(from: String, ride_offer: RideOffer) -> Transaction {
        let function_call = FunctionCall {
            name: "rideOffer".to_string(),
            arguments: serde_json::to_string(&ride_offer).unwrap() 
        };

        Transaction::new_tranaction(from, function_call)
    }   

    pub fn new_ride_accept_tranaction(from:String, ride_acceptance:RideAcceptance) ->Transaction{
        let function_call = FunctionCall{
            name : "rideAcceptance".to_string(),
            arguments : serde_json::to_string(&ride_acceptance).unwrap(),
        };

        Transaction::new_tranaction(from, function_call)
    }

    pub fn new_confirm_arrival_tranaction(from:String, confirm_arrival:ConfirmArrival) -> Transaction{
        let function_call: FunctionCall = FunctionCall{
            name: "confirmArrival".to_string(),
            arguments: serde_json::to_string(&confirm_arrival).unwrap(),
        };

        Transaction::new_tranaction(from, function_call)
    }

    pub fn new_complain_arrival_tranaction(from:String, complain_arrival:ComplainArrival) -> Transaction{
        let function_call: FunctionCall = FunctionCall{
            name: "complainArrival".to_string(),
            arguments: serde_json::to_string(&complain_arrival).unwrap(),
        };

        Transaction::new_tranaction(from, function_call)
    }

    pub fn new_ride_payment_tranaction(from:String, ride_payment:RidePayment) -> Transaction{
        let function_call: FunctionCall = FunctionCall{
            name: "ridePayment".to_string(),
            arguments: serde_json::to_string(&ride_payment).unwrap(),
        };

        Transaction::new_tranaction(from, function_call)
    }
}