use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::vec;

use clutch_node::node::{block::Block, blockchain::Blockchain, function_call::FunctionCallType, *};

const BLOCKCHAIN_NAME: &str = "clutch-node-test";

const PASSENGER_ADDRESS_KEY: &str = "0xdeb4cfb63db134698e1879ea24904df074726cc0";
const PASSENGER_SECRET_KEY: &str =
    "d2c446110cfcecbdf05b2be528e72483de5b6f7ef9c7856df2f81f48e9f2748f";

const DRIVER_ADDRESS_KEY: &str = "0x8f19077627cde4848b090c53c83b12956837d5e9";
const DRIVER_SECRET_KEY: &str = "e74e3f87268132c7b3ddb24600716fc362f4519bf9986a9436aa8a1be58c7150";

const RIDE_REQUEST_TX_HASH: &str =
    "70d4cd23a2fc6c636ed1ac7744a7d58869ec95f7066d8441645821a0420f0164";
const RIDE_OFFER_TX_HASH: &str = "c72839a57eeb93971409828845ef0b443ccb8f50a18ebf9559dba39c639633a7";
const RIDE_ACCEPTANCE_TX_HASH: &str =
    "856a5dae6fee5f249dbd144321ca28badd9297088d4927af27069e37a8cccdd9";

const AUTHOR_1_PUBLIC_KEY: &str = "0x9b6e8afff8329743cac73dbef83ca3cbf9a74c20";
const AUTHOR_1_SECRET_KEY: &str =
    "0883ddd3d07303b87c954b0c9383f7b78f45e002520fc03a8adc80595dbf6509";

const AUTHOR_2_PUBLIC_KEY: &str = "0x6fc11ba44483201f6e9c5eba6435805bb94ad080";
const AUTHOR_2_SECRET_KEY: &str =
    "9aba0d89bfa358d27cfc119657537b9c92c8e38a35d2333ddd5c62e6d1a9b15e";

const AUTHOR_3_PUBLIC_KEY: &str = "0xc4f3f661a43e099aedb8e396d9de1a831a1b4adc";
const AUTHOR_3_SECRET_KEY: &str =
    "2d75bdfabbbaa65d7a182968e579adf2566fbb6931411752dd834c56bbf092c9";

#[test]
fn blocks_imports() {
    let mut blockchain = new_blockchain();

    let blocks = [
        || ride_request_block(1, 1, 20),
        || ride_offer_block(2, 1, 30),
        || ride_acceptance_block(3, 2),
        || ride_pay_block(4, 3, 5),  //5
        || ride_pay_block(5, 4, 10), // 5 + 10 = 15
        || ride_pay_block(6, 5, 10), // 15 + 10 = 25
        || ride_cancel_block(7, 6),
    ];

    for block_creator in blocks.iter() {
        let mut block = block_creator();
        if let Err(e) = import_block(&mut blockchain, &mut block) {
            println!("Error importing block: {}", e);
            break;
        }
    }

    let latest_block = blockchain
        .get_latest_block()
        .expect("Failed to get the latest block");

    println!(
        "Blockchain name: {:#?}, latest block index: {}",
        blockchain.name, latest_block.index,
    );

    let from_account_state = blockchain.get_account_state(&PASSENGER_ADDRESS_KEY.to_string());
    println!("From account state: {:#?}", from_account_state);

    save_blocks_to_file(&blockchain);
    blockchain.cleanup_if_developer_mode();
}

fn new_blockchain() -> Blockchain {
    let authorities = vec![
        AUTHOR_1_PUBLIC_KEY.to_string(),
        AUTHOR_2_PUBLIC_KEY.to_string(),
        AUTHOR_3_PUBLIC_KEY.to_string(),
    ];
    let blockchain = Blockchain::new(BLOCKCHAIN_NAME.to_string(), true, authorities);
    blockchain
}

fn import_block(blockchain: &mut Blockchain, block: &mut Block) -> Result<(), String> {
    block.previous_hash = blockchain
        .get_latest_block()
        .expect("Failed to get the latest block")
        .hash;

    if let Some((public_key, secret_key)) = current_author_keys(blockchain) {
        block.sign(public_key, secret_key);
    } else {
        return Err("Current author not found".to_string());
    }

    blockchain.block_import(block)
}

fn current_author_keys(blockchain: &Blockchain) -> Option<(&str, &str)> {
    let author_keys = [
        (AUTHOR_1_PUBLIC_KEY, AUTHOR_1_SECRET_KEY),
        (AUTHOR_2_PUBLIC_KEY, AUTHOR_2_SECRET_KEY),
        (AUTHOR_3_PUBLIC_KEY, AUTHOR_3_SECRET_KEY),
    ];

    let current_author = blockchain.current_author();

    for &(public_key, secret_key) in &author_keys {
        if current_author == public_key {
            return Some((public_key, secret_key));
        }
    }
    None
}

fn save_blocks_to_file(blockchain: &Blockchain) {
    let path = Path::new("output/ride_sharing_sample.json");
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

fn ride_request_block(index: usize, nonce: u64, fare: u64) -> Block {
    let ride_request_transcation = ride_request_transcation(fare, nonce);
    Block::new_block(index, String::new(), vec![ride_request_transcation])
}

fn ride_request_transcation(fare: u64, nonce: u64) -> transaction::Transaction {
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

    let mut ride_request_transcation = transaction::Transaction::new_transaction(
        PASSENGER_ADDRESS_KEY.to_string(),
        nonce,
        FunctionCallType::RideRequest,
        ride_request,
    );

    ride_request_transcation.sign(PASSENGER_SECRET_KEY);
    ride_request_transcation
}

fn ride_offer_block(index: usize, nonce: u64, fare: u64) -> Block {
    let ride_offer_transaction: transaction::Transaction = ride_offer_transaction(fare, nonce);
    Block::new_block(index, String::new(), vec![ride_offer_transaction])
}

fn ride_offer_transaction(fare: u64, nonce: u64) -> transaction::Transaction {
    let ride_offer = ride_offer::RideOffer {
        fare: fare,
        ride_request_transaction_hash: RIDE_REQUEST_TX_HASH.to_string(),
    };

    let mut ride_offer_transaction = transaction::Transaction::new_transaction(
        DRIVER_ADDRESS_KEY.to_string(),
        nonce,
        FunctionCallType::RideOffer,
        ride_offer,
    );
    ride_offer_transaction.sign(DRIVER_SECRET_KEY);
    ride_offer_transaction
}

fn ride_acceptance_block(index: usize, nonce: u64) -> Block {
    let ride_acceptance_transaction = ride_acceptance_transaction(nonce);
    Block::new_block(index, String::new(), vec![ride_acceptance_transaction])
}

fn ride_acceptance_transaction(nonce: u64) -> transaction::Transaction {
    let ride_acceptance = ride_acceptance::RideAcceptance {
        ride_offer_transaction_hash: RIDE_OFFER_TX_HASH.to_string(),
    };

    let mut ride_acceptance_transaction = transaction::Transaction::new_transaction(
        PASSENGER_ADDRESS_KEY.to_string(),
        nonce,
        FunctionCallType::RideAcceptance,
        ride_acceptance,
    );
    ride_acceptance_transaction.sign(PASSENGER_SECRET_KEY);
    ride_acceptance_transaction
}

fn ride_pay_block(index: usize, nonce: u64, fare: u64) -> Block {
    let ride_pay_transaction = ride_pay_transaction(fare, nonce);
    Block::new_block(index, String::new(), vec![ride_pay_transaction])
}

fn ride_pay_transaction(fare: u64, nonce: u64) -> transaction::Transaction {
    let ride_pay = ride_pay::RidePay {
        fare: fare,
        ride_acceptance_transaction_hash: RIDE_ACCEPTANCE_TX_HASH.to_string(),
    };

    let mut ride_pay_transaction = transaction::Transaction::new_transaction(
        PASSENGER_ADDRESS_KEY.to_string(),
        nonce,
        FunctionCallType::RidePay,
        ride_pay,
    );
    ride_pay_transaction.sign(PASSENGER_SECRET_KEY);
    ride_pay_transaction
}

fn ride_cancel_block(index: usize, nonce: u64) -> Block {
    let ride_cancel_transaction = ride_cancel_transaction(nonce);
    Block::new_block(index, String::new(), vec![ride_cancel_transaction])
}

fn ride_cancel_transaction(nonce: u64) -> transaction::Transaction {
    let ride_pay = ride_cancel::RideCancel {
        ride_acceptance_transaction_hash: RIDE_ACCEPTANCE_TX_HASH.to_string(),
    };

    let mut ride_cancel_transaction = transaction::Transaction::new_transaction(
        PASSENGER_ADDRESS_KEY.to_string(),
        nonce,
        FunctionCallType::RideCancel,
        ride_pay,
    );

    ride_cancel_transaction.sign(PASSENGER_SECRET_KEY);
    ride_cancel_transaction
}
