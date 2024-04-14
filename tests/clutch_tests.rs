use clutch_node::node::blockchain::Blockchain;
use clutch_node::node::function_call::FunctionCallType;
use clutch_node::node::{block::Block, *};

const BLOCKCHAIN_NAME: &str = "clutch-node-test";
const FROM: &str = "0xb87a9ac289f679f1f489fefa14f885187e311e2f";
const TO: &str = "0xa300e57228487edb1f5c0e737cbfc72d126b5bc2";

#[test]
fn test() {
    let mut blockchain = Blockchain::new(BLOCKCHAIN_NAME.to_string(), true);

    let block1 = transfer_block(1);
    blockchain.block_import(block1);

    // let block2 = ride_request_block();
    // blockchain.block_import(block2);

    // let block3 = ride_offer_block();
    // blockchain.block_import(block3);

    println!("Blockchain: {:#?}", blockchain);
    blockchain.cleanup_if_developer_mode();
}

fn transfer_block(index: usize) -> Block {
    let transfer = transfer::Transfer {
        to: TO.to_string(),
        value: 110.0,
    };
    let transfer_request_transcation = transaction::Transaction::new_transaction(
        FROM.to_string(),
        0,
        FunctionCallType::Transfer,
        transfer,
    );
    let transfer_request_block = Block::new_block(index, vec![transfer_request_transcation]);
    transfer_request_block
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
        FROM.to_string(),
        0,
        FunctionCallType::Transfer,
        ride_request,
    );
    let ride_request_block = Block::new_block(index, vec![ride_request_transcation]);
    ride_request_block
}
