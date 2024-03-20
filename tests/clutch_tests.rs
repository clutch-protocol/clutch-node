use clutch_node::node::{block::Block, *};

const FROM: &str = "0xMehran"; 

#[test]
fn test(){
    
    let ride_request = ride_request::RideRequest{
        pickup_location : coordinate::Coordinates { latitude: 35.55841414973938, longitude: 51.23861773552397 },//Home
        dropoff_location : coordinate::Coordinates { latitude: 26.649646426996483, longitude: 55.857706441083984 }  //Ghil,hengam iceland
    };
    let ride_request_transcation = transaction::Transaction::ride_request(FROM.to_string(), ride_request);
    let ride_request_block= block::Block::new_block(vec![ride_request_transcation]);
    
    let blocks:Vec<Block> = vec![ride_request_block];
    let _blockchain = blockchain::Blockchain::new_from_blocks(blocks);    
} 