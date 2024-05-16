use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::vec;

use clutch_node::node::{
    block::Block, blockchain::Blockchain, function_call::FunctionCallType,
    *,
};

const BLOCKCHAIN_NAME: &str = "clutch-node-test";
const FROM_ADDRESS_KEY: &str = "0xdeb4cfb63db134698e1879ea24904df074726cc0";
const FROM_SECRET_KEY: &str = "d2c446110cfcecbdf05b2be528e72483de5b6f7ef9c7856df2f81f48e9f2748f";
const TO: &str = "0xa300e57228487edb1f5c0e737cbfc72d126b5bc2";
const RIDE_REQUEST_TX_HASH: &str =
    "939368c8bc84dc2b10286a420ee7100568d4c36a7f6308d1c4b29f0f77b4e83b";
const RIDE_OFFER_TX_HASH: &str = "b32250c25c42cd25f9b9af99285fe9ec434ed260e5b426dac47dd820fedd06b5";
const RIDE_ACCEPTANCE_TX_HASH: &str =
    "d269c2ec9d03df1450e7330c776dd814cbafb61a6652a68c945be1c185508f65";

#[test]
fn test() {
    let mut blockchain = Blockchain::new(BLOCKCHAIN_NAME.to_string(), true);

    // Import multiple blocks using an array of closure functions
    let blocks = [
        || transfer_block(1, 1, 10),
        || ride_request_block(2, 2, 20),
        || ride_offer_block(3, 3, 30),
        || ride_acceptance_block(4, 4),
        || ride_pay_block(5, 5, 5), //5
        || ride_pay_block(6, 6, 10), // 5+10 = 15
        || ride_pay_block(7, 7, 10), // 15 + 10 = 25
        || ride_cancel_block(8, 8),
        // || ride_pay_block(9, 9, 5),
    ];

    // Iterate over the block creation functions, modify and import each block
    for block_creator in blocks.iter() {
        let mut block = block_creator();
        if let Err(e) = import_block(&mut blockchain, &mut block) {
            println!("Error importing block: {}", e);
            continue;
        }
    }

    // Output the blockchain status
    let latest_block = blockchain
        .get_latest_block()
        .expect("Failed to get the latest block");

    println!(
        "Blockchain name: {:#?}, latest block index: {}",
        blockchain.name, latest_block.index,
    );

    // Output the from account status
    let from_account_state = blockchain.get_current_state(&FROM_ADDRESS_KEY.to_string());
    println!("From account state: {:#?}", from_account_state);

    // Save and cleanup tasks
    save_blocks_to_file(&blockchain);
    blockchain.cleanup_if_developer_mode();
}

fn import_block(blockchain: &mut Blockchain, block: &mut Block) -> Result<(), String> {
    // Update the previous hash and import the block
    block.previous_hash = blockchain
        .get_latest_block()
        .expect("Failed to get the latest block")
        .hash;
    blockchain.block_import(block)
}

fn save_blocks_to_file(blockchain: &Blockchain) {
    let path = Path::new("output/blcoks.json");
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to create file: {}", e);
            return;
        }
    };

    match blockchain.get_blocks() {
        Ok(blocks) => {
            match serde_json::to_string_pretty(&blocks) {
                Ok(json_str) => {
                    if let Err(e) = writeln!(file, "{}", json_str) {
                        println!("Failed to write to file: {}", e);
                        return;
                    }
                }
                Err(e) => {
                    println!("Failed to serialize block: {}", e);
                    return;
                }
            }
            println!(
                "Blocks have been successfully saved to '{}'.",
                path.display()
            );
        }
        Err(e) => {
            println!("Failed to retrieve blocks: {}", e);
        }
    }
}

fn transfer_block(index: usize, nonce: u64, transfer_value: u64) -> Block {
    let transfer = transfer::Transfer {
        to: TO.to_string(),
        value: transfer_value,
    };

    let transfer_request_transcation = transaction::Transaction::new_transaction(
        FROM_ADDRESS_KEY.to_string(),
        nonce,
        FunctionCallType::Transfer,
        FROM_SECRET_KEY.to_string(),
        transfer,
    );

    Block::new_block(index, String::new(), vec![transfer_request_transcation])
}

fn ride_request_block(index: usize, nonce: u64, fare: u64) -> Block {
    let ride_request = ride_request::RideRequest {
        fare: fare,
        pickup_location: coordinate::Coordinates {
            latitude: 35.55841414973938,
            longitude: 51.23861773552397,
        }, //Tehran,Iran
        dropoff_location: coordinate::Coordinates {
            latitude: 26.649646426996483,
            longitude: 55.857706441083984,
        }, //Ghil,Hengam iceland,Iran
    };

    let ride_request_transcation = transaction::Transaction::new_transaction(
        FROM_ADDRESS_KEY.to_string(),
        nonce,
        FunctionCallType::RideRequest,
        FROM_SECRET_KEY.to_string(),
        ride_request,
    );

    Block::new_block(index, String::new(), vec![ride_request_transcation])
}

fn ride_offer_block(index: usize, nonce: u64, fare: u64) -> Block {
    let ride_offer = ride_offer::RideOffer {
        fare: fare,
        ride_request_transaction_hash: RIDE_REQUEST_TX_HASH.to_string(),
    };

    let ride_offer_transaction = transaction::Transaction::new_transaction(
        FROM_ADDRESS_KEY.to_string(),
        nonce,
        FunctionCallType::RideOffer,
        FROM_SECRET_KEY.to_string(),
        ride_offer,
    );

    Block::new_block(index, String::new(), vec![ride_offer_transaction])
}

fn ride_acceptance_block(index: usize, nonce: u64) -> Block {
    let ride_acceptance = ride_acceptance::RideAcceptance {
        ride_offer_transaction_hash: RIDE_OFFER_TX_HASH.to_string(),
    };

    let ride_acceptance_transaction = transaction::Transaction::new_transaction(
        FROM_ADDRESS_KEY.to_string(),
        nonce,
        FunctionCallType::RideAcceptance,
        FROM_SECRET_KEY.to_string(),
        ride_acceptance,
    );

    Block::new_block(index, String::new(), vec![ride_acceptance_transaction])
}

fn ride_pay_block(index: usize, nonce: u64, fare: u64) -> Block {
    let ride_pay = ride_pay::RidePay {
        fare: fare,
        ride_acceptance_transaction_hash: RIDE_ACCEPTANCE_TX_HASH.to_string(),
    };

    let ride_pay_transaction = transaction::Transaction::new_transaction(
        FROM_ADDRESS_KEY.to_string(),
        nonce,
        FunctionCallType::RidePay,
        FROM_SECRET_KEY.to_string(),
        ride_pay,
    );

    Block::new_block(index, String::new(), vec![ride_pay_transaction])
}

fn ride_cancel_block(index: usize, nonce: u64) -> Block {
    let ride_pay = ride_cancel::RideCancel {
        ride_acceptance_transaction_hash: RIDE_ACCEPTANCE_TX_HASH.to_string(),
    };

    let ride_cancel_transaction = transaction::Transaction::new_transaction(
        FROM_ADDRESS_KEY.to_string(),
        nonce,
        FunctionCallType::RideCancel,
        FROM_SECRET_KEY.to_string(),
        ride_pay,
    );

    Block::new_block(index, String::new(), vec![ride_cancel_transaction])
}
