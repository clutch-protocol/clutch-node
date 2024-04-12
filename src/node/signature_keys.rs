use std::fmt::format;

use rand::rngs::OsRng;
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey};
use sha2::Digest as Sha256Digest;
use sha2::Sha256;
use sha3::Digest as Keccak256Digest;
use sha3::Keccak256;

#[derive(Debug)]
pub struct SignatureKeys {
    secret_key: SecretKey,
    public_key: PublicKey,
    address_key: String,
}

impl SignatureKeys {
    fn derive_address(public_key: &PublicKey) -> String {
        let serialized_pubkey = public_key.serialize_uncompressed();
        let mut hasher = Keccak256::new();
        hasher.update(&serialized_pubkey[1..]);
        let hash = hasher.finalize();

        let address_key = format!("0x{}", hex::encode(&hash[12..32]));
        address_key
    }

    fn create_message_digest(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        result.into()
    }

    pub fn generate_new_keypair() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::default();
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        let address_key = Self::derive_address(&public_key);

        SignatureKeys {
            secret_key: secret_key,
            public_key: public_key,
            address_key: address_key,
        }
    }

    pub fn sign(&self, data: &[u8]) -> Signature {
        let secp = Secp256k1::new();
        let message_digest = Self::create_message_digest(data); // Hashing the data first
        let message = Message::from_digest_slice(&message_digest).expect("32 bytes");

        secp.sign_ecdsa(&message, &self.secret_key)
    }

    pub fn verify(public_key_str: &String, data: &[u8], signature: &Signature) -> bool {
        let secp = Secp256k1::new();

        let message_digest = Self::create_message_digest(data); // Hashing the data first
        let message = Message::from_digest_slice(&message_digest).expect("32 bytes");

        // Decode the public key from a hex string
        let public_key_bytes = match hex::decode(&public_key_str) {
            Ok(bytes) => bytes,
            Err(_) => {
                println!("Failed to decode public key from hex string");
                return false;
            }
        };

        // Convert the decoded bytes into a PublicKey
        let public_key = match PublicKey::from_slice(&public_key_bytes) {
            Ok(pk) => pk,
            Err(_) => {
                println!("Failed to create public key from slice");
                return false;
            }
        };

        secp.verify_ecdsa(&message, signature, &public_key).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_new_keypair() {
        let keys = SignatureKeys::generate_new_keypair();
        println!(
            "{:?},{:?},{:?}",
            keys.address_key,
            keys.secret_key.display_secret(),
            keys.public_key
        )
    }

    #[test]
    fn test_sign_and_verify() {
        let keys = SignatureKeys::generate_new_keypair();
        let data = b"Blockchain technology";
        println!("Public key:{:?}", keys.public_key);
        println!("Address:{:?}", keys.address_key);

        // Test signing
        let signature = keys.sign(data);
        println!("Signature:{:?}", signature);

        // Instead of comparing against a default, verify the signature directly
        let secp = Secp256k1::new();
        let message_digest = SignatureKeys::create_message_digest(data);
        let message = Message::from_digest_slice(&message_digest).expect("32 bytes");
        println!("message:{:?}", &message);

        assert!(
            secp.verify_ecdsa(&message, &signature, &keys.public_key)
                .is_ok(),
            "Signature should be valid and verifiable"
        );

        // Test verification with correct data
        assert!(
            SignatureKeys::verify(&keys.public_key.to_string(), data, &signature),
            "Signature should be valid"
        );

        // Test verification with incorrect data
        let incorrect_data = b"Wrong data";
        assert!(
            !SignatureKeys::verify(&keys.public_key.to_string(), incorrect_data, &signature),
            "Signature should be invalid with incorrect data"
        );
    }
}
