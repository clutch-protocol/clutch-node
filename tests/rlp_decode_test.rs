#[cfg(test)]
mod tests {
    use clutch_node::node::transactions::function_call::FunctionCall;
    use clutch_node::node::transactions::ride_request::RideRequest;
    use hex;    
    use clutch_node::node::{coordinate, rlp_encoding};
    use clutch_node::node::transactions::transaction::Transaction;
    use std::str::from_utf8;
    const PASSENGER_ADDRESS_KEY: &str = "0xdeb4cfb63db134698e1879ea24904df074726cc0";
    const PASSENGER_SECRET_KEY: &str ="d2c446110cfcecbdf05b2be528e72483de5b6f7ef9c7856df2f81f48e9f2748f";    
   
    #[test]
    fn decode_rlp_to_transaction_struct() {
        // Example RLP-encoded transaction hex (replace with your actual test vector if needed)        
        let rlp_hex = "0xf90131a86465623463666236336462313334363938653138373965613234393034646630373437323663633002b84031393732656538663962313463353137373337613334666230356534613935313336653537393162646530383661326532393066356133633435626435366631b840303964323532616239646461653065343537313062313263363261623266376334346135383663393566303132343430303961646361343039663865633532621bb84063353833306139343962656264666539306362376236663635666530353634623334656631666533333339333739663838366234343838613666386530303831f83e81eb0181e981d28188403b3481bc21734e608188404c2a81d56981f281983381d28188403b3881a4819a0b457e8188404c2d785081f381853681820381e8";
        let rlp_bytes = hex::decode(rlp_hex.trim_start_matches("0x")).expect("Invalid hex");

        // Debug print: show each RLP field
        let rlp = rlp::Rlp::new(&rlp_bytes);
        println!("RLP item count: {}", rlp.item_count().unwrap_or(0));
        
        // Enhanced debugging to understand the structure better
        println!("Top level is list: {}", rlp.is_list());
        
        // Investigate each field to find any RLP structure issues
        for i in 0..rlp.item_count().unwrap_or(0) {
            let val = rlp.at(i).unwrap();
            
            // Get the bytes directly
            if let Ok(data) = val.data() {
                if let Ok(str_val) = from_utf8(data) {
                    println!("Field {}: String({:?}), bytes: {}", i, str_val, hex::encode(data));
                } else {
                    println!("Field {}: Binary, bytes: {}", i, hex::encode(data));
                }
            } else if val.is_list() {
                println!("Field {}: List with {} items", i, val.item_count().unwrap_or(0));
                
                // If this is field 6 (data field), print more details
                if i == 6 {
                    println!("  Data field structure:");
                    // Check if it follows the expected structure [tag, args]
                    if val.item_count().unwrap_or(0) >= 2 {
                        if let Ok(tag) = val.at(0).unwrap().as_val::<u8>() {
                            println!("  Tag: {}", tag);
                        }
                        
                        let args = val.at(1).unwrap();
                        if args.is_list() {
                            println!("  Args is a list with {} items", args.item_count().unwrap_or(0));
                        } else {
                            println!("  Args is not a list");
                        }
                    }
                }
            } else {
                println!("Field {}: Unknown type", i);
            }
        }

        // Decode to Transaction struct
        match rlp_encoding::decode::<Transaction>(&rlp_bytes) {
            Ok(tx) => println!("Decoded Transaction: {:#?}", tx),
            Err(e) => {
                println!("Failed to decode RLP to Transaction: {:?}", e);
                // Print more details about expected structure
                println!("Expected RLP structure for Transaction:");
                println!("- 7 items in top-level list");
                println!("- Fields: [from, nonce, signature_r, signature_s, signature_v, hash, data]");
                println!("- 'data' should be a list [tag, args] where:");
                println!("  - tag is a u8 (0-7) indicating function call type");
                println!("  - args varies depending on tag");
            },
        }
    }

    
#[test]
fn test_rlp_encode_ride_request_transaction() {
    // Create a sample RideRequest transaction and print its RLP encoding
    let ride_request = RideRequest {
        pickup_location: coordinate::Coordinates {
            latitude: 27.223374842000805,
            longitude: 56.365535283043855,
        },
        dropoff_location: coordinate::Coordinates {
            latitude: 27.225817157860583,
            longitude: 56.40913096554422,
        },
        fare: 1000,
    };
    // Use nonce 1 for example
    let mut tx = Transaction::new_transaction(
        PASSENGER_ADDRESS_KEY.to_string(),
        1,
        FunctionCall::RideRequest(ride_request),
    );
    // Sign with passenger's secret key
    tx.sign(PASSENGER_SECRET_KEY);
    // Encode to RLP
    let encoded = clutch_node::node::rlp_encoding::encode(&tx);
    println!("RideRequest Tx RLP: 0x{}", hex::encode(&encoded));
    
    // Also print the decoded version to verify structure
    println!("\nVerifying by decoding our own encoding:");
    match rlp_encoding::decode::<Transaction>(&encoded) {
        Ok(decoded_tx) => println!("Successfully decoded our own transaction: {:?}", decoded_tx),
        Err(e) => println!("Failed to decode our own transaction: {:?}", e),
    }
}
} 