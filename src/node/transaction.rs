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
    pub pickup_location: String,
    pub dropoff_location: String,
    pub passenger: String, // Consider adding more fields as necessary
}

impl Transaction{
    pub fn new_genesis_transactions() -> Vec<Transaction> {
        vec![]
    }

      pub fn ride_request(from: String, to: String, request: RideRequest) -> Transaction {
        let function_call = FunctionCall {
            name: "rideRequest".to_string(),
            arguments: vec![
                request.pickup_location,
                request.dropoff_location,
                request.passenger,
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

        // Create a sample ride request
        let ride_request = RideRequest {
            pickup_location: "123 Main St".to_string(),
            dropoff_location: "456 Elm St".to_string(),
            passenger: "Charlie".to_string(),
        };

        let transaction = Transaction::ride_request(from_address.clone(), to_address.clone(), ride_request);

        assert_eq!(transaction.from, from_address);
        assert_eq!(transaction.to, to_address);
        assert_eq!(transaction.data.name, "rideRequest".to_string());
        assert_eq!(transaction.data.arguments, vec!["123 Main St", "456 Elm St", "Charlie"]);

    }
}
