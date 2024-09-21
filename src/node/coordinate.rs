use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Encodable for Coordinates {
    fn rlp_append(&self, stream: &mut RlpStream) {
        // Begin an RLP list with two elements: latitude and longitude
        stream.begin_list(2);
        // Convert f64 to u64 using to_bits()
        let latitude_bits = self.latitude.to_bits();
        let longitude_bits = self.longitude.to_bits();
        stream.append(&latitude_bits);
        stream.append(&longitude_bits);
    }
}

impl Decodable for Coordinates {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        // Decode latitude and longitude as u64
        let latitude_bits: u64 = rlp.val_at(0)?;
        let longitude_bits: u64 = rlp.val_at(1)?;
        // Convert u64 bits back to f64
        let latitude = f64::from_bits(latitude_bits);
        let longitude = f64::from_bits(longitude_bits);
        Ok(Coordinates {
            latitude,
            longitude,
        })
    }
}