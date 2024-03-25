use std::vec;
use sha2::{Sha256, Digest};
use serde::{Deserialize,Serialize};
use crate::node::ride_request::RideRequest;
use crate::node::ride_offer::RideOffer;
use crate::node::ride_acceptance::RideAcceptance;
use crate::node::confirm_arrival::ConfirmArrival;
use crate::node::complain_arrival::ComplainArrival;
use crate::node::ride_payment::RidePayment;
use crate::node::transfer::Transfer;
use crate::node::function_call::{FunctionCall,FunctionCallType};

const FROM_GENESIS: &str = "0xGENESIS";

#[derive(Debug,Serialize,Deserialize)]
pub struct Transaction {
    pub from: String, 
    pub hash: String,
    pub data: FunctionCall,
}

impl Transaction {
    pub fn new_genesis_transactions() -> Vec<Transaction> {

    let tx1=  Self:: new_transfer_transaction(FROM_GENESIS.to_string(), Transfer{to: "0xb87a9ac289f679f1f489fefa14f885187e311e2f".to_string(), value:10.0}); 
    let tx2=  Self:: new_transfer_transaction(FROM_GENESIS.to_string(), Transfer{to: "0xa300e57228487edb1f5c0e737cbfc72d126b5bc2".to_string(), value:10.0}); 
    let tx3=  Self:: new_transfer_transaction(FROM_GENESIS.to_string(), Transfer{to: "0xac20ff4e42ff243046faaf032068762dd2c018dc".to_string(), value:10.0}); 
    let tx4=  Self:: new_transfer_transaction(FROM_GENESIS.to_string(), Transfer{to: "0xa91101310bee451ca0e219aba08d8d4dd929f16c".to_string(), value:10.0}); 
    let tx5=  Self:: new_transfer_transaction(FROM_GENESIS.to_string(), Transfer{to: "0x37adf81cb1f18762042e5da03a55f1e54ba66870".to_string(), value:10.0}); 

     vec![tx1,tx2,tx3,tx4,tx5]
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", self.from, self.data.function_call_type, self.data.arguments));
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    fn new_tranaction(from: String, function_call: FunctionCall) -> Transaction {
        let mut transaction = Transaction {         
                hash: String::new(),   
                from:from,           
                data: function_call,
            };
    
        transaction.hash = transaction.calculate_hash();
        transaction
    }

    pub fn validate_transactions(transactions: &Vec<Transaction>) -> bool {
        for tx in transactions {
            let is_valid = match tx.data.function_call_type {
                FunctionCallType::Transfer => {
                    // Add validation logic for Transfer
                    // e.g., check sender's balance, verify digital signature, etc.
                    true // Return true if valid, false otherwise
                },
                FunctionCallType::RideRequest => {
                    // Validation logic for RideRequest
                    true
                },
                FunctionCallType::RideOffer => {
                    // Validation logic for RideOffer
                    true
                },
                FunctionCallType::RideAcceptance => {
                    // Validation logic for RideAcceptance
                    true
                },
                FunctionCallType::ConfirmArrival => {
                    // Validation logic for ConfirmArrival
                    true
                },
                FunctionCallType::ComplainArrival => {
                    // Validation logic for ComplainArrival
                    true
                },
                FunctionCallType::RidePayment => {
                    // Validation logic for RidePayment
                    // This might include checking the ride status, confirming the fare, etc.
                    true
                },
                _ => false // Add more types as needed
            };

            // If any transaction is invalid, return false immediately
            if !is_valid {
                return false;
            }
        }

        // If all transactions are valid, return true
        true
    }

    pub fn new_transfer_transaction(from: String, transfer: Transfer) -> Transaction {
        let function_call = FunctionCall {
            function_call_type: FunctionCallType::Transfer,
            arguments: serde_json::to_string(&transfer).unwrap()
        };

        Transaction::new_tranaction(from, function_call)
    }

    pub fn new_ride_request_tranaction(from: String, ride_request: RideRequest) -> Transaction {
        let function_call = FunctionCall {
            function_call_type: FunctionCallType::RideRequest,
            arguments: serde_json::to_string(&ride_request).unwrap()
        };

        Transaction::new_tranaction(from, function_call)
    }

    pub fn new_ride_offer_tranaction(from: String, ride_offer: RideOffer) -> Transaction {
        let function_call = FunctionCall {
            function_call_type: FunctionCallType::RideOffer,
            arguments: serde_json::to_string(&ride_offer).unwrap() 
        };

        Transaction::new_tranaction(from, function_call)
    }   

    pub fn new_ride_accept_tranaction(from:String, ride_acceptance:RideAcceptance) ->Transaction{
        let function_call = FunctionCall {            
            function_call_type: FunctionCallType::RideAcceptance,
            arguments : serde_json::to_string(&ride_acceptance).unwrap(),
        };

        Transaction::new_tranaction(from, function_call)
    }

    pub fn new_confirm_arrival_tranaction(from:String, confirm_arrival:ConfirmArrival) -> Transaction{
        let function_call: FunctionCall = FunctionCall{            
            function_call_type: FunctionCallType::ConfirmArrival,
            arguments: serde_json::to_string(&confirm_arrival).unwrap(),
        };

        Transaction::new_tranaction(from, function_call)
    }

    pub fn new_complain_arrival_tranaction(from:String, complain_arrival:ComplainArrival) -> Transaction{
        let function_call: FunctionCall = FunctionCall{            
            function_call_type: FunctionCallType::ComplainArrival,
            arguments: serde_json::to_string(&complain_arrival).unwrap(),
        };

        Transaction::new_tranaction(from, function_call)
    }

    pub fn new_ride_payment_tranaction(from:String, ride_payment:RidePayment) -> Transaction{
        let function_call: FunctionCall = FunctionCall{            
            function_call_type: FunctionCallType::RidePayment,
            arguments: serde_json::to_string(&ride_payment).unwrap(),
        };

        Transaction::new_tranaction(from, function_call)
    }
}