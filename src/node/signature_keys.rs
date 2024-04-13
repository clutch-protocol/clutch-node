use hex::FromHex;
use rand::rngs::OsRng;
use secp256k1::{ecdsa::RecoveryId, ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

#[derive(Debug)]
pub struct SignatureKeys {
    secret_key: String,
    public_key: String,
    address_key: String,
}

impl SignatureKeys {
    pub fn generate_new_keypair() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::default();
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        let address_key = Self::derive_address(&public_key);

        SignatureKeys {
            secret_key: hex::encode(secret_key.as_ref()),
            public_key: hex::encode(public_key.serialize_uncompressed()),
            address_key: address_key,
        }
    }

    fn derive_address(public_key: &PublicKey) -> String {
        let serialized_pubkey = public_key.serialize_uncompressed();
        let mut hasher = Keccak256::new();
        hasher.update(&serialized_pubkey[1..]);
        let hash = hasher.finalize();

        let address_key = format!("0x{}", hex::encode(&hash[12..32]));
        address_key
    }

    pub fn sign(secret_key: &str, data: &[u8]) -> (String, String, i32) {
        let secp = Secp256k1::new();
        let secret_key_bytes = hex::decode(secret_key).unwrap();
        let secret_key = SecretKey::from_slice(&secret_key_bytes).unwrap();

        // Create a message hash (Keccak-256 of the data)
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let message_hash = hasher.finalize();

        // Create a message object for secp256k1
        let message = Message::from_digest_slice(&message_hash)
            .expect("Message could not be created from hash");

        // Sign the message
        let recoverable_sig = secp.sign_ecdsa_recoverable(&message, &secret_key);

        // Serialize the signature to compact format
        let (recid, sig) = recoverable_sig.serialize_compact();

        // Convert signature and recovery ID to appropriate formats
        let r = hex::encode(&sig[0..32]); // r component
        let s = hex::encode(&sig[32..64]); // s component
        let v = recid.to_i32() + 27; // recovery ID, adjusted for Ethereum (v = 27 or 28)

        (r, s, v)
    }

    pub fn verify(public_key_hex: &str, data: &[u8], r: &str, s: &str, v: i32) -> bool {
        let secp = Secp256k1::new();
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let message_hash = hasher.finalize();
        let message = Message::from_digest_slice(&message_hash)
            .expect("Message could not be created from hash");

        let public_key_bytes = hex::decode(public_key_hex).expect("Invalid hex for public key");
        let public_key = PublicKey::from_slice(&public_key_bytes).expect("Invalid public key");

        let sig_r = Vec::from_hex(r).expect("Invalid hex in r");
        let sig_s = Vec::from_hex(s).expect("Invalid hex in s");
        let recovery_id = RecoveryId::from_i32(v - 27).expect("Invalid recovery ID");
        let signature = Signature::from_compact(&[&sig_r[..], &sig_s[..]].concat())
            .expect("Invalid signature format");

        secp.verify_ecdsa(&message, &signature, &public_key).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use crate::node::signature_keys;

    use super::*;

    #[test]
    fn test_generate_new_keypair() {
        let keys = SignatureKeys::generate_new_keypair();
        println!(
            "{:?},{:?},{:?}",
            keys.address_key, keys.secret_key, keys.public_key
        )
    }

    #[test]
    fn test_sign_and_verify() {
        let keys = SignatureKeys::generate_new_keypair();
        let data = b"Blockchain technology";
        println!("Public key:{:?}", keys.public_key);
        println!("Address:{:?}", keys.address_key);

        // Test signing
        let (r, s, v) = SignatureKeys::sign(&keys.secret_key, data);
        println!("Signature: r={:?}, s={:?} , v={:?}", r, s, v);

        let is_verified  = SignatureKeys::verify(
            &keys.public_key,
            data,
            &r,
            &s,
            v,
        );

        assert!(is_verified, "Signature verification should succeed");
    }

    #[test]
    fn test_sign_and_verify_failure_on_modified_data() {
        let keys = SignatureKeys::generate_new_keypair();
        let original_data = b"Blockchain technology";
        let modified_data = b"Altered data";

        // Test signing with the original data
        let (r, s, v) = SignatureKeys::sign(&keys.secret_key, original_data);            

        // Attempt to verify signature against modified data
        let is_verified = SignatureKeys::verify(&keys.public_key, modified_data, &r, &s, v);
        assert!(!is_verified, "Signature verification should fail on modified data");
    }

    #[test]
    fn test_sign_and_verify_failure_on_wrong_key() {
        let keys = SignatureKeys::generate_new_keypair();
        let other_keys = SignatureKeys::generate_new_keypair();  // Generate a different key pair
        let data = b"Blockchain technology";

        // Test signing with the first key
        let (r, s, v) = SignatureKeys::sign(&keys.secret_key, data);          

        // Attempt to verify signature with a different public key
        let is_verified = SignatureKeys::verify(&other_keys.public_key, data, &r, &s, v);
        assert!(!is_verified, "Signature verification should fail with a different public key");
    }

}
