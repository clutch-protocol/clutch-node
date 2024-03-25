
use secp256k1::{Secp256k1, SecretKey,Message, PublicKey,ecdsa::{Signature}};
use rand::rngs::OsRng; 
use sha2::{Sha256, Digest};
use hex;

pub fn create_message_digest(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.into()
}

#[derive(Debug)]
pub struct SignatureKeys{
    secret_key: SecretKey,
    public_key: PublicKey,
}

impl SignatureKeys{
    pub fn generate_new_keypair()-> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::default();
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        SignatureKeys{
            secret_key : secret_key,
            public_key : public_key,
        }
    }

    pub fn sign(&self, data: &[u8]) -> Signature {
        let secp = Secp256k1::new();
        let message_digest = create_message_digest(data); // Hashing the data first
        let message = Message::from_digest_slice(&message_digest).expect("32 bytes");

        secp.sign_ecdsa(&message, &self.secret_key)
    }

    pub fn verify(&self, data: &[u8], signature: &Signature) -> bool {
        let secp = Secp256k1::new();

        let message_digest = create_message_digest(data); // Hashing the data first
        let message = Message::from_digest_slice(&message_digest).expect("32 bytes");
        
        secp.verify_ecdsa(&message, signature, &self.public_key).is_ok()
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::ecdsa::Signature;

    #[test]
    fn test_generate_new_keypair() {
        let keys = SignatureKeys::generate_new_keypair();
        println!("{:?},{:?}",keys.public_key,keys.secret_key)

    }

    #[test]
    fn test_sign_and_verify() {
        let keys = SignatureKeys::generate_new_keypair();
        let data = b"Blockchain technology";

        // Test signing
        let signature = keys.sign(data);

        // Instead of comparing against a default, verify the signature directly
        let secp = Secp256k1::new();
        let message_digest = create_message_digest(data);
        let message = Message::from_digest_slice(&message_digest).expect("32 bytes");
        assert!(secp.verify_ecdsa(&message, &signature, &keys.public_key).is_ok(), "Signature should be valid and verifiable");

        // Test verification with correct data
        assert!(keys.verify(data, &signature), "Signature should be valid");

        // Test verification with incorrect data
        let incorrect_data = b"Wrong data";
        assert!(!keys.verify(incorrect_data, &signature), "Signature should be invalid with incorrect data");
    } // due to its cryptographic nature; often, tests will simulate this by using a different key to sign or altering the data.
}
