
use secp256k1::{Secp256k1, SecretKey,Message, PublicKey,ecdsa::{Signature}};
use rand::rngs::OsRng; 
use sha2::{Sha256, Digest};

pub fn create_message_digest(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.into()
}

#[derive(Debug)]
pub struct SignatureKey{
    secret_key: SecretKey,
    public_key: PublicKey,
}

impl SignatureKey{
    pub fn generate_new_keypair()-> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::default();
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        SignatureKey{
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
    use super::*; // Import everything from the outer module.

    #[test]
    fn test_generate_new_keypair() {
        // Generate the keypair
        let sig = SignatureKey::generate_new_keypair();

        // Check that the secret key is not empty
        //assert!(!sig.secret_key.public_key(secp).is_empty(), "Secret key should not be empty");

        // Check that the public key is not empty
        //assert!(!sig.public_key.is_empty(), "Public key should not be empty");

        // Additional checks could include format validations, length checks, etc.
        // For example, checking if the public key starts with "04" if uncompressed (typical for Secp256k1)
        // assert!(sig.public_key.starts_with("04"), "Public key should start with '04'");
        
        println!("Public Key: {:?}", sig.public_key);
        println!("Secret Key: {:?}", sig.secret_key);
    }
}