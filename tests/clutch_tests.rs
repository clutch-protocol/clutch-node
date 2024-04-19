use std::fs::File;
use std::io::Write; // To use the write! macro for writing to a file
use std::path::Path;
use std::vec;

use clutch_node::node::blockchain::Blockchain;
use clutch_node::node::function_call::FunctionCallType;
use clutch_node::node::ride_offer::RideOffer;
use clutch_node::node::{block::Block, *};

const BLOCKCHAIN_NAME: &str = "clutch-node-test";
// const FROM_PUBLIC_KEY: &str = "04a5ddc16b93f7e744fbab3c025cf99a0ef00113c6727353a3dff406fb4d136a06d73244619adc980818931da1b053462ef5af5e121cb5616be45325edd9b0be15";
const FROM_ADDRESS_KEY: &str = "0xdeb4cfb63db134698e1879ea24904df074726cc0";
const FROM_SECRET_KEY: &str = "d2c446110cfcecbdf05b2be528e72483de5b6f7ef9c7856df2f81f48e9f2748f";

const TO: &str = "0xa300e57228487edb1f5c0e737cbfc72d126b5bc2";

const RIDE_REQUEST_TX_HASH: &str =
    "02724637e27d8aba2057605a6f6d10607b5921cee81ffc9980484fb5b555f183";

#[test]
fn test() {
    let mut blockchain = Blockchain::new(BLOCKCHAIN_NAME.to_string(), true);

    let block_1 = transfer_block(1);
    blockchain.block_import(&block_1);

    let block_2 = ride_request_block(2);
    blockchain.block_import(&block_2);

    let block_3 = ride_offer_block(3, RIDE_REQUEST_TX_HASH);
    blockchain.block_import(&block_3);

    println!(
        "Blockchain name: {:#?}, latest block index: {}",
        blockchain.name,
        blockchain.get_latest_block_index(),
    );

    save_blocks_to_file(&blockchain);

    blockchain.cleanup_if_developer_mode();
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
            for block in blocks {
                match serde_json::to_string_pretty(&block) {
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

fn transfer_block(index: usize) -> Block {
    let transfer = transfer::Transfer {
        to: TO.to_string(),
        value: 100.0,
    };

    let transfer_request_transcation = transaction::Transaction::new_transaction(
        FROM_ADDRESS_KEY.to_string(),
        1,
        FunctionCallType::Transfer,
        FROM_SECRET_KEY.to_string(),
        transfer,
    );

    Block::new_block(index, vec![transfer_request_transcation])
}

fn ride_request_block(index: usize) -> Block {
    let ride_request = ride_request::RideRequest {
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
        2,
        FunctionCallType::RideRequest,
        FROM_SECRET_KEY.to_string(),
        ride_request,
    );

    Block::new_block(index, vec![ride_request_transcation])
}

fn ride_offer_block(index: usize, ride_request_tx_hash: &str) -> Block {
    let ride_offer = ride_offer::RideOffer {
        fare: 1000,
        ride_request_transaction_hash: RIDE_REQUEST_TX_HASH.to_string(),
    };

    let ride_offer_transaction = transaction::Transaction::new_transaction(
        FROM_ADDRESS_KEY.to_string(),
        3,
        FunctionCallType::RideOffer,
        FROM_SECRET_KEY.to_string(),
        ride_offer,
    );

    Block::new_block(index, vec![ride_offer_transaction])
}
