use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::vec;

use clutch_node::node::{block::Block, blockchain::Blockchain, function_call::FunctionCallType, *};

const BLOCKCHAIN_NAME: &str = "clutch-node-test";

const FROM_ADDRESS_KEY: &str = "0xdeb4cfb63db134698e1879ea24904df074726cc0";
const FROM_SECRET_KEY: &str = "d2c446110cfcecbdf05b2be528e72483de5b6f7ef9c7856df2f81f48e9f2748f";

const TO_ADDRESS_KEY: &str = "0x8f19077627cde4848b090c53c83b12956837d5e9";

#[test]
fn transfer_founds() {
    let mut blockchain = Blockchain::new(BLOCKCHAIN_NAME.to_string(), true);

    let blocks = [|| transfer_block(1, 1, 20)];

    for block_creator in blocks.iter() {
        let mut block = block_creator();
        if let Err(e) = import_block(&mut blockchain, &mut block) {
            println!("Error importing block: {}", e);
            continue;
        }
    }

    let latest_block = blockchain
        .get_latest_block()
        .expect("Failed to get the latest block");

    println!(
        "Blockchain name: {:#?}, latest block index: {}",
        blockchain.name, latest_block.index,
    );

    let from_account_state = blockchain.get_current_state(&FROM_ADDRESS_KEY.to_string());
    println!("From account state: {:#?}", from_account_state);

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
        to: TO_ADDRESS_KEY.to_string(),
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
