use clutch_node::node::{block::Block, *};

const FROM: &str = "0xMehran"; 

#[test]
fn test(){    

    let block1 = ride_request_block();
    print!("From: {} ",block1.transactions[0].hash);
    let block2 = ride_offer_block();


    let blocks:Vec<Block> = vec![block1,block2];
    let _blockchain = blockchain::Blockchain::new_from_blocks(blocks);    
}

fn ride_request_block() -> Block {
    let ride_request = ride_request::RideRequest{
        pickup_location : coordinate::Coordinates { latitude: 35.55841414973938, longitude: 51.23861773552397 }, //Tehran,Iran
        dropoff_location : coordinate::Coordinates { latitude: 26.649646426996483, longitude: 55.857706441083984 } //Ghil,Hengam iceland,Iran
    };
    let ride_request_transcation = transaction::Transaction::new_ride_request_tranaction(FROM.to_string(), ride_request);
    let ride_request_block= block::Block::new_block(vec![ride_request_transcation]);
    ride_request_block
}

fn ride_offer_block() -> Block {
    let ride_offer = ride_offer::RideOffer{
       fare: 1440,
       ride_request_transaction_hash  : "".to_string(),
    };
    let ride_request_transcation = transaction::Transaction::new_ride_offer_tranaction(FROM.to_string(), ride_offer);
    let ride_offer_block= block::Block::new_block(vec![ride_request_transcation]);
    ride_offer_block
}