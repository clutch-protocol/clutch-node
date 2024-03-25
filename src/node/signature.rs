
use serde::{Deserialize,Serialize};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use secp256k1::rand::rngs::OsRng;

#[derive(Debug,Serialize,Deserialize)]
pub struct signature{
    secret_key : String,
    public_key : String,
}

impl signature{
    pub fn generate_new_keypair()-> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        signature{
            secret_key : secret_key.display_secret().to_string(),
            public_key : public_key.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module.

    #[test]
    fn test_generate_new_keypair() {
        // Generate the keypair
        let sig = signature::generate_new_keypair();

        // Check that the secret key is not empty
        assert!(!sig.secret_key.is_empty(), "Secret key should not be empty");

        // Check that the public key is not empty
        assert!(!sig.public_key.is_empty(), "Public key should not be empty");

        // Additional checks could include format validations, length checks, etc.
        // For example, checking if the public key starts with "04" if uncompressed (typical for Secp256k1)
        // assert!(sig.public_key.starts_with("04"), "Public key should start with '04'");
        
        println!("Secret Key: {}", sig.secret_key);
        println!("Public Key: {}", sig.public_key);
    }
}