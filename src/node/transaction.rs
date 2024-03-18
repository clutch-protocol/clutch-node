use crate::node::coordinate::Coordinates;

pub struct Transaction {
    pub from: String,
    pub to: String,
    pub value: f64, 
    pub data: FunctionCall,
}

pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<String>,
}

pub struct RideRequest {
    pub pickup_location: Coordinates,
    pub dropoff_location: Coordinates,
}

impl Transaction{
    pub fn new_genesis_transactions() -> Vec<Transaction> {
        vec![]
    }

      pub fn ride_request(from: String, to: String, request: RideRequest) -> Transaction {
        let function_call = FunctionCall {
            name: "rideRequest".to_string(),
            arguments: vec![
                request.pickup_location.latitude.to_string(),
                request.pickup_location.longitude.to_string(),  
                request.dropoff_location.latitude.to_string(),
                request.dropoff_location.longitude.to_string(),
            ],
        };

        Transaction {
            from:from,
            to: to,
            value: 0.0,
            data: function_call,
        }
    }
}

mod tests{    
    use super::*; 

    #[test]
    fn new_ride_request(){
        let from_address = "Alice".to_string();
        let to_address = "Bob".to_string();

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

        let transaction = Transaction::ride_request(from_address.clone(), to_address.clone(), ride_request);

        assert_eq!(transaction.from, from_address);
        assert_eq!(transaction.to, to_address);
        assert_eq!(transaction.data.name, "rideRequest".to_string());
        assert_eq!(transaction.data.arguments, vec!["40.712776", "-74.005974","40.712776", "-73.986397"]);
    }
}
