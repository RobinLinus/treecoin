extern crate ring;
extern crate untrusted;
extern crate serde;

use self::ring::{rand, signature};

use hex::{to_hex};

use types::{ U160, U256 };

#[derive(Clone)]
pub struct PrivateKey([u8;85]);

impl PrivateKey {
    pub fn to_hex(&self) -> String{
        to_hex(self.0.to_vec())
    }

    pub fn bytes(&self) -> &[u8;85]{
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PublicKey(pub U256);

impl PublicKey {
    pub fn from_bytes(bytes: &[u8]) -> PublicKey{
        let mut array = [0; 32];
        let bytes = &bytes[..array.len()];
        array.copy_from_slice(bytes);
        PublicKey(array)
    }

    pub fn to_address(&self)->Address{
        Address::from_public_key(self)
    }

    pub fn to_hex(&self) -> String{
        to_hex(self.0.to_vec())
    }

    pub fn bytes(&self) -> &U256{
        &self.0
    }
}

pub struct Address(pub U160);

impl Address {
    pub fn from_public_key(public_key:&PublicKey) -> Address{
        let mut array = [0; 20];
        let bytes = &public_key.bytes()[..array.len()];
        array.copy_from_slice(bytes);
        Address(array)
    }

    pub fn to_hex(&self) -> String{
        to_hex(self.0.to_vec())
    }
}

pub struct KeyPair {
    pub public_key: PublicKey,
    pub private_key: PrivateKey
}

impl KeyPair {
    pub fn generate() -> Result<KeyPair, ring::error::Unspecified> {
        let rng = rand::SystemRandom::new();
        
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;

        let key_pair = signature::Ed25519KeyPair::from_pkcs8(
                untrusted::Input::from(&pkcs8_bytes))?;
        
        Ok(KeyPair {
            public_key : PublicKey::from_bytes(key_pair.public_key_bytes()),
            private_key : PrivateKey(pkcs8_bytes),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_generation() {
        let keys = KeyPair::generate().unwrap();
        let address = keys.public_key.to_address();
        println!("Public Key: {:?}", keys.public_key.to_hex());
        println!("Private Key: {:?}", keys.private_key.to_hex());
        println!("Address: {:?}", address.to_hex());
    }
}