use sha2::{Sha256, Digest};
use crate::node::rideRequest::RideRequest;

pub struct Transaction {
    pub from: String,
    pub to: Option<String>,
    pub value: f64, 
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
        hasher.update(format!("{}{}{}{}{}", self.from, self.to, self.value, self.data.name, self.data.arguments));
        let result = hasher.finalize();
        format!("{:x}", result)                 
    }

    pub fn new_genesis_transactions() -> Vec<Transaction> {
        vec![]
    }

    pub fn ride_request(from: String, request: RideRequest) -> Transaction {

        let function_call = FunctionCall {
            name: "rideRequest".to_string(),
            arguments: serde_json::to_string(&request).unwrap()
        };

       let mut transaction = Transaction {         
            hash: String::new(),   
            from:from,
            to: None,
            value: 0.0,
            data: function_call,
        };

        transaction.hash = transaction.calculate_hash();
        transaction
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
