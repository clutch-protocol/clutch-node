use clutch_node::node::blockchain::Blockchain;
use clutch_node::node::function_call::FunctionCallType;
use clutch_node::node::{block::Block, *};

const BLOCKCHAIN_NAME: &str = "clutch-node-test";
const FROM_PUBLIC_KEY: &str = "04a5ddc16b93f7e744fbab3c025cf99a0ef00113c6727353a3dff406fb4d136a06d73244619adc980818931da1b053462ef5af5e121cb5616be45325edd9b0be15";
const FROM_ADDRESS_KEY: &str = "0xdeb4cfb63db134698e1879ea24904df074726cc0";
const FROM_SECRET_KEY: &str = "d2c446110cfcecbdf05b2be528e72483de5b6f7ef9c7856df2f81f48e9f2748f";

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
        FROM_ADDRESS_KEY.to_string(),
        0,
        FunctionCallType::Transfer,
        FROM_SECRET_KEY.to_string(),
        transfer,
    );
    let transfer_request_block = Block::new_block(index, vec![transfer_request_transcation]);
    transfer_request_block
}