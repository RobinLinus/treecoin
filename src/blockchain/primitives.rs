use std::fmt;
use std::fmt::Debug;
use network::message::Writeable;
use std::io::Write;
use std::io::Error;
use network::message::Readable;
use std::io::Read;


use utils::hex::{to_hex,from_hex};

pub struct Hash([u8;32]);

impl Hash {
    pub fn new(buffer: [u8;32]) -> Hash {
	    Hash(buffer)
	}

	pub fn from_bytes(bytes: &[u8])->Hash{
		let mut hash:[u8;32] = Default::default();
    	hash[..32].clone_from_slice(&bytes);
    	Hash::new(hash)
	}

	pub fn to_hex(&self) -> String{
		to_hex(self.0.to_vec())
	}
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl Writeable for Hash {
    fn write(&self, writer: &mut Write) -> Result<(), Error>{
	    writer.write(&self.0);
	    Ok(())
	}
} 

impl Readable for Hash {
    fn read(reader: &mut Read) -> Result<Hash, Error>{
    	let mut buffer = [0u8;32];
		Self::read_fixed_size(reader, &mut buffer);
		Ok(Hash::new(buffer))
	}
}


pub mod message_type{
	pub const VER: 		u32 = 0;
	pub const VER_ACK: 	u32 = 1;
	pub const BLOCK: 	u32 = 2;
	pub const ADDRESS: 	u32 = 3;
}
