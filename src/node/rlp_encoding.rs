extern crate rlp;

use crate::node::block::Block;
use crate::node::function_call::{FunctionCall, FunctionCallType};
use crate::node::handshake::Handshake;
use crate::node::transaction::Transaction;

use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

use super::block_bodies::BlockBodies;
use super::block_headers::{BlockHeader, BlockHeaders};
use super::get_block_bodies::GetBlockBodies;
use super::get_block_header::GetBlockHeaders;

impl Encodable for FunctionCallType {
    fn rlp_append(&self, stream: &mut RlpStream) {
        let value = match *self {
            FunctionCallType::Transfer => 0u8,
            FunctionCallType::RideRequest => 1u8,
            FunctionCallType::RideOffer => 2u8,
            FunctionCallType::RideAcceptance => 3u8,
            FunctionCallType::RidePay => 4u8,
            FunctionCallType::RideCancel => 5u8,
            FunctionCallType::ConfirmArrival => 6u8,
            FunctionCallType::ComplainArrival => 7u8,
        };
        stream.append(&value);
    }
}

impl Decodable for FunctionCallType {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        match rlp.as_val::<u8>()? {
            0 => Ok(FunctionCallType::Transfer),
            1 => Ok(FunctionCallType::RideRequest),
            2 => Ok(FunctionCallType::RideOffer),
            3 => Ok(FunctionCallType::RideAcceptance),
            4 => Ok(FunctionCallType::RidePay),
            5 => Ok(FunctionCallType::RideCancel),
            6 => Ok(FunctionCallType::ConfirmArrival),
            7 => Ok(FunctionCallType::ComplainArrival),
            _ => Err(DecoderError::Custom("Unknown FunctionCallType")),
        }
    }
}

impl Encodable for FunctionCall {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream.begin_list(2);
        stream.append(&self.arguments);
        stream.append(&self.function_call_type);
    }
}
impl Decodable for FunctionCall {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(FunctionCall {
            arguments: rlp.val_at(0)?,
            function_call_type: rlp.val_at(1)?,
        })
    }
}

impl Encodable for Transaction {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream.begin_list(7);

        stream.append(&self.from);
        stream.append(&self.nonce);
        stream.append(&self.signature_r);
        stream.append(&self.signature_s);
        let signature_v_as_u64 = self.signature_v as u64;
        stream.append(&signature_v_as_u64);
        stream.append(&self.hash);
        stream.append(&self.data);
    }
}

impl Decodable for Transaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(Transaction {
            from: rlp.val_at(0)?,
            nonce: rlp.val_at(1)?,
            signature_r: rlp.val_at(2)?,
            signature_s: rlp.val_at(3)?,
            signature_v: rlp.val_at::<u64>(4)? as i32,
            hash: rlp.val_at(5)?,
            data: rlp.val_at(6)?,
        })
    }
}

impl Encodable for Block {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream.begin_list(9);

        stream.append(&self.index);
        stream.append(&self.timestamp);
        stream.append(&self.previous_hash);
        stream.append(&self.author);
        stream.append(&self.signature_r);
        stream.append(&self.signature_s);
        let signature_v_as_u64 = self.signature_v as u64;
        stream.append(&signature_v_as_u64);
        stream.append(&self.hash);
        stream.append_list(&self.transactions);
    }
}

impl Decodable for Block {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(Block {
            index: rlp.val_at(0)?,
            timestamp : rlp.val_at(1)?,
            previous_hash: rlp.val_at(2)?,
            author: rlp.val_at(3)?,
            signature_r: rlp.val_at(4)?,
            signature_s: rlp.val_at(5)?,
            signature_v: rlp.val_at::<u64>(6)? as i32,
            hash: rlp.val_at(7)?,
            transactions: rlp.list_at(8)?,
        })
    }
}

impl Encodable for Handshake {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream.begin_list(2);
        stream.append(&self.genesis_block_hash);
        stream.append(&self.latest_block_hash);
        stream.append(&self.latest_block_index);
    }
}

impl Decodable for Handshake {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(Handshake {
            genesis_block_hash: rlp.val_at(0)?,
            latest_block_hash: rlp.val_at(1)?,
            latest_block_index: rlp.val_at(2)?,
        })
    }
}

impl Encodable for GetBlockHeaders {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream.begin_list(3);
        stream.append(&self.start_block_index);
        stream.append(&self.skip);
        stream.append(&self.limit);
    }
}

impl Decodable for GetBlockHeaders {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(GetBlockHeaders {
            start_block_index: rlp.val_at(0)?,
            skip: rlp.val_at(1)?,
            limit: rlp.val_at(2)?,
        })
    }
}

impl Encodable for BlockHeader {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream.begin_list(7);
        stream.append(&self.index);
        stream.append(&self.previous_hash);
        stream.append(&self.author);
        stream.append(&self.signature_r);
        stream.append(&self.signature_s);
        let signature_v_as_u64 = self.signature_v as u64;
        stream.append(&signature_v_as_u64);
        stream.append(&self.hash);
    }
}

impl Decodable for BlockHeader {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(BlockHeader {
            index: rlp.val_at(0)?,
            previous_hash: rlp.val_at(1)?,
            author: rlp.val_at(2)?,
            signature_r: rlp.val_at(3)?,
            signature_s: rlp.val_at(4)?,
            signature_v: rlp.val_at::<u64>(5)? as i32,
            hash: rlp.val_at(6)?,
        })
    }
}

impl Encodable for BlockHeaders {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream.begin_list(1);
        stream.append_list(&self.block_headers);
    }
}

impl Decodable for BlockHeaders {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(BlockHeaders {
            block_headers: rlp.list_at(0)?,
        })
    }
}

impl Encodable for GetBlockBodies {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream.begin_list(1);
        stream.append_list(&self.block_indexes);
    }
}

impl Decodable for GetBlockBodies {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(GetBlockBodies {
            block_indexes: rlp.list_at(0)?,
        })
    }
}

impl Encodable for BlockBodies {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream.begin_list(1);
        stream.append_list(&self.blocks);
    }
}

impl Decodable for BlockBodies {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(BlockBodies {
            blocks: rlp.list_at(0)?,
        })
    }
}

pub fn encode<T: Encodable>(data: &T) -> Vec<u8> {
    let mut stream = RlpStream::new();
    data.rlp_append(&mut stream);
    stream.out().to_vec()
}

pub fn decode<T: Decodable>(bytes: &[u8]) -> Result<T, DecoderError> {
    let rlp = Rlp::new(bytes);
    T::decode(&rlp)
}

#[cfg(test)]
mod tests {

    use tracing::{error, info};

    use crate::node::time_utils::get_current_timespan;

    use super::*;

    #[test]
    fn test_encode_decode_transaction() {
        let tx = Transaction {
            from: "0xdeb4cfb63db134698e1879ea24904df074726cc0".to_string(),
            data: FunctionCall {
                function_call_type: FunctionCallType::Transfer,
                arguments: "{\"to\":\"0x8f19077627cde4848b090c53c83b12956837d5e9\",\"value\":10}"
                    .to_string(),
            },
            nonce: 1,
            signature_r: "3b0cb46ae73d852bb75653ed1f1710676b0b736cd33aefc0c96e6e11417a4c32"
                .to_string(),
            signature_s: "296086bdc703286c0727c59e07b727cadfc2fe7b9c061149e4a86e726ed23908"
                .to_string(),
            signature_v: 27,
            hash: "0086095648e3160d0dfa5d40bdf4693d8a00d77ed3fb3b607156465b3e0de2db".to_string(),
        };

        let encoded = encode(&tx);
        info!("Encoded: {:?}", encoded);

        let decoded = decode::<Transaction>(&encoded);
        match decoded {
            Ok(tx) => info!("Decoded: {:?}", tx),
            Err(e) => error!("Failed to decode transaction: {:?}", e),
        }
    }

    #[test]
    fn test_encode_decode_block() {
        let tx1 = Transaction {
            from: "0xdeb4cfb63db134698e1879ea24904df074726cc0".to_string(),
            data: FunctionCall {
                function_call_type: FunctionCallType::Transfer,
                arguments: "{\"to\":\"0x8f19077627cde4848b090c53c83b12956837d5e9\",\"value\":10}"
                    .to_string(),
            },
            nonce: 1,
            signature_r: "3b0cb46ae73d852bb75653ed1f1710676b0b736cd33aefc0c96e6e11417a4c32"
                .to_string(),
            signature_s: "296086bdc703286c0727c59e07b727cadfc2fe7b9c061149e4a86e726ed23908"
                .to_string(),
            signature_v: 27,
            hash: "0086095648e3160d0dfa5d40bdf4693d8a00d77ed3fb3b607156465b3e0de2db".to_string(),
        };

        let tx2 = Transaction {
            from: "0xabc4cfb63db134698e1879ea24904df074726cc0".to_string(),
            data: FunctionCall {
                function_call_type: FunctionCallType::RideRequest,
                arguments: "{\"to\":\"0x1f19077627cde4848b090c53c83b12956837d5e9\",\"value\":5}"
                    .to_string(),
            },
            nonce: 2,
            signature_r: "2b0cb46ae73d852bb75653ed1f1710676b0b736cd33aefc0c96e6e11417a4c33"
                .to_string(),
            signature_s: "396086bdc703286c0727c59e07b727cadfc2fe7b9c061149e4a86e726ed23909"
                .to_string(),
            signature_v: 28,
            hash: "1086095648e3160d0dfa5d40bdf4693d8a00d77ed3fb3b607156465b3e0de2db".to_string(),
        };

        let block = Block {
            index: 1,
            timestamp : get_current_timespan(),
            previous_hash: "0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            author: "0x1234cfb63db134698e1879ea24904df074726cc0".to_string(),
            signature_r: "4b0cb46ae73d852bb75653ed1f1710676b0b736cd33aefc0c96e6e11417a4c34"
                .to_string(),
            signature_s: "496086bdc703286c0727c59e07b727cadfc2fe7b9c061149e4a86e726ed23910"
                .to_string(),
            signature_v: 27,
            hash: "2086095648e3160d0dfa5d40bdf4693d8a00d77ed3fb3b607156465b3e0de2dc".to_string(),
            transactions: vec![tx1, tx2],
        };

        let encoded = encode(&block);
        info!("Encoded Block: {:?}", encoded);

        let decoded = decode::<Block>(&encoded);
        match decoded {
            Ok(block) => info!("Decoded Block: {:?}", block),
            Err(e) => error!("Failed to decode block: {:?}", e),
        }
    }

    #[test]
    fn test_encode_decode_get_block_headers() {
        let get_block_headers = GetBlockHeaders {
            start_block_index: 0,
            skip: 0,
            limit: 100,
        };

        let encoded = encode(&get_block_headers);
        info!("Encoded: {:?}", encoded);

        let decoded = decode::<GetBlockHeaders>(&encoded);
        match decoded {
            Ok(tx) => info!("Decoded: {:?}", tx),
            Err(e) => error!("Failed to decode transaction: {:?}", e),
        }
    }

    #[test]
    fn test_encode_decode_block_headers() {
        let block_header_1 = BlockHeader {
            index: 1,
            previous_hash: "0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            author: "0x1234cfb63db134698e1879ea24904df074726cc0".to_string(),
            signature_r: "4b0cb46ae73d852bb75653ed1f1710676b0b736cd33aefc0c96e6e11417a4c34"
                .to_string(),
            signature_s: "496086bdc703286c0727c59e07b727cadfc2fe7b9c061149e4a86e726ed23910"
                .to_string(),
            signature_v: 27,
            hash: "2086095648e3160d0dfa5d40bdf4693d8a00d77ed3fb3b607156465b3e0de2dc".to_string(),
        };

        let block_header_2 = BlockHeader {
            index: 1,
            previous_hash: "0000000000000000000000000000000000000000000000000000000000000002"
                .to_string(),
            author: "0x1234cfb63db134698e1879ea24904df074726cc0".to_string(),
            signature_r: "4b0cb46ae73d852bb75653ed1f1710676b0b736cd33aefc0c96e6e11417a4c34"
                .to_string(),
            signature_s: "496086bdc703286c0727c59e07b727cadfc2fe7b9c061149e4a86e726ed23910"
                .to_string(),
            signature_v: 27,
            hash: "2086095648e3160d0dfa5d40bdf4693d8a00d77ed3fb3b607156465b3e0de2dc".to_string(),
        };

        let block_headers = BlockHeaders {
            block_headers: vec![block_header_1, block_header_2],
        };

        let encoded = encode(&block_headers);
        info!("Encoded: {:?}", encoded);

        let decoded = decode::<BlockHeaders>(&encoded);
        match decoded {
            Ok(tx) => info!("Decoded: {:?}", tx),
            Err(e) => error!("Failed to decode BlockHeaders: {:?}", e),
        }
    }

    #[test]
    fn test_encode_decode_block_bodies() {
        let block = Block {
            index: 1,
            timestamp: get_current_timespan(),
            previous_hash: "0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            author: "0x1234cfb63db134698e1879ea24904df074726cc0".to_string(),
            signature_r: "4b0cb46ae73d852bb75653ed1f1710676b0b736cd33aefc0c96e6e11417a4c34"
                .to_string(),
            signature_s: "496086bdc703286c0727c59e07b727cadfc2fe7b9c061149e4a86e726ed23910"
                .to_string(),
            signature_v: 27,
            hash: "2086095648e3160d0dfa5d40bdf4693d8a00d77ed3fb3b607156465b3e0de2dc".to_string(),
            transactions: vec![],
        };

        let block_boodies = BlockBodies {
            blocks: vec![block],
        };

        let encoded = encode(&block_boodies);
        info!("Encoded: {:?}", encoded);

        let decoded = decode::<BlockBodies>(&encoded);
        match decoded {
            Ok(tx) => info!("Decoded: {:?}", tx),
            Err(e) => error!("Failed to decode BlockBodies: {:?}", e),
        }
    }
}
