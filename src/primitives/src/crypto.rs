extern crate blake2_rfc as blake2;
extern crate ring;
extern crate untrusted;


use hex::{to_hex};
use types::{U256, U512, Bytes};

use keys::{KeyPair,PublicKey, PrivateKey, Address};
use crypto::blake2::blake2b::{blake2b};
use self::ring::{signature};



pub struct Signature(U512);

impl Signature {
    pub fn from_bytes(bytes: &Bytes) -> Signature{
        let mut signature:[u8;64] = [0;64];
        signature[..64].clone_from_slice(&bytes);
        Signature(signature)
    }

    pub fn bytes(&self) -> &U512 {
        &self.0
    }

    pub fn to_hex(&self) -> String{
        to_hex(self.0.to_vec())
    }
}

pub struct Hash(U256);

impl Hash {
    pub fn from_bytes(bytes: &Bytes) -> Hash{
    	let mut hash:[u8;32] = Default::default();
    	hash[..32].clone_from_slice(&bytes);
    	Hash(hash)
    }
    pub fn to_hex(&self) -> String{
        to_hex(self.0.to_vec())
    }
}


pub struct Crypto {}

impl Crypto {
    pub fn hash(buffer: &Bytes) -> Hash {
        let bytes = blake2b(32, &[], buffer);
        Hash::from_bytes(bytes.as_bytes())
    }
    
    pub fn generate_keys() -> KeyPair {
        KeyPair::generate().unwrap()
    }

    pub fn sign(message: &Bytes, private_key: &PrivateKey) -> Signature {
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(private_key.bytes())).unwrap();
        let sig = key_pair.sign(message);
        Signature::from_bytes(sig.as_ref())
    }

    pub fn verify(message: &Bytes, signature: &Signature, public_key: &PublicKey) -> bool {
        let signature = (&signature).bytes();
        let public_key = (&public_key).bytes();
        let key = untrusted::Input::from(public_key);
        let msg = untrusted::Input::from(message);
        let sig = untrusted::Input::from(signature);
        match signature::verify(&signature::ED25519, key, msg, sig) {
            Ok(_v) => true,
            Err(_e) => false,
        }
    }
}


pub struct Witness {
    public_key: PublicKey,
    signature: Signature
}

impl Witness {
    pub fn new(private_key: &PrivateKey, message: &Bytes) -> Witness{
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(private_key.bytes())).unwrap();
        let sig = key_pair.sign(message);
        let public_key = PublicKey::from_bytes(key_pair.public_key_bytes());
        let signature = Signature::from_bytes(sig.as_ref());
        Witness{
            public_key: public_key,
            signature: signature
        }
    }

    pub fn verify(&self, message: &Bytes) -> bool{
        Crypto::verify(message, &self.signature, &self.public_key)
    }

    pub fn address(&self) -> Address{
        self.public_key.to_address()
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hash_strings() {
        let buffer: &Bytes = b"Is Elon Musk Satoshi?";
        let hash = Crypto::hash(buffer);
        let hex = hash.to_hex();
        println!("Hash: {:?}", hex );
        assert_eq!(hex, "237c0104076008f95d7ba64d6813fa842733950d86820e7190f50c3c712a19c2");
    }

    #[test]
    fn sign_and_verify(){
        let message: &[u8] = b"Is Elon Musk Satoshi?";
        let keys = Crypto::generate_keys();
        let fake_keys = Crypto::generate_keys();
        let signature = Crypto::sign(message, &keys.private_key);
        println!("Signature: {:?}", signature.to_hex() );
        assert!(Crypto::verify(message, &signature, &keys.public_key));
        assert_eq!(Crypto::verify(message, &signature, &fake_keys.public_key),false);
    }

    #[test]
    fn witness(){
        let message: &[u8] = b"Is Elon Musk Satoshi?";
        let keys = Crypto::generate_keys();
        let fake_keys = Crypto::generate_keys();
        let witness = Witness::new(&keys.private_key, message);
        assert!(witness.verify(message));
    }
}