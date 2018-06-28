extern crate blake2_rfc as blake2;
use utils::crypto::blake2::blake2b::blake2b;
use blockchain::primitives::Hash;

pub struct Crypto;



impl Crypto {
    pub fn hash(buffer: &[u8]) -> Hash{
    	let bytes = blake2b(32, &[], buffer);
        Hash::from_bytes(bytes.as_bytes())
    }
}