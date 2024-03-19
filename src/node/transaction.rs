use sha2::{Sha256, Digest};
use crate::node::rideRequest::RideRequest;
use crate::node::rideOffer::RideOffer;
use crate::node::rideAcceptance::RideAcceptance;
use crate::node::confirmArrival::ConfirmArrival;

use super::confirmArrival;

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

    pub fn ride_request(from: String, ride_request: RideRequest) -> Transaction {
        let function_call = FunctionCall {
            name: "rideRequest".to_string(),
            arguments: serde_json::to_string(&ride_request).unwrap()
        };

        Transaction::new_tranaction(from, function_call)
    }

    pub fn ride_offer(from: String, ride_offer: RideOffer) -> Transaction {
        let function_call = FunctionCall {
            name: "rideOffer".to_string(),
            arguments: serde_json::to_string(&ride_offer).unwrap() 
        };

        Transaction::new_tranaction(from, function_call)
    }   

    pub fn ride_accept(from:String, ride_acceptance:RideAcceptance) ->Transaction{
        let function_call = FunctionCall{
            name : "rideAcceptance".to_string(),
            arguments : serde_json::to_string(&ride_acceptance).unwrap(),
        };

        Transaction::new_tranaction(from, function_call)
    }

    pub fn confirm_arrival(from:String, confirm_arrival:ConfirmArrival) -> Transaction{
        let function_call = FunctionCall{
            name: "confirmArrival".to_string(),
            arguments: serde_json::to_string(&confirm_arrival).unwrap(),
        };

        Transaction::new_tranaction(from, function_call)
    }
}



mod tests{    
    use super::*; 
    use crate::node::coordinate::Coordinates;

    #[test]
    fn new_ride_request(){
        let from_address = "Alice".to_string();

        let ride_request = RideRequest {
            pickup_location: Coordinates {
                latitude: 40.712776,
                longitude : -74.005974,
            },
            dropoff_location: Coordinates {
                latitude: 40.712776,
                longitude : -73.986397,
            }        
        };

        let serilized= serde_json::to_string(&ride_request).unwrap();
        let transaction = Transaction::ride_request(from_address.clone(), ride_request);

        assert_eq!(transaction.from, from_address);
        assert_eq!(transaction.data.name, "rideRequest".to_string());
        assert_eq!(transaction.data.arguments,serilized);
        //print!("{}",serilized);
    }
}
