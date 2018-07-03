extern crate blake2_rfc as blake2;
use std::mem::transmute;
use utils::hash::blake2::blake2b::Blake2b;
extern crate rand;
use utils::serializer::{ Reader, Readable, Writer, Writeable };
use std::fmt;
use std::io::{ Error };
use utils::hex::{to_hex};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Hash([u8;32]);

static ZEROS : Hash = Hash([0u8; 32]);

impl Hash {
    pub fn new(buffer: [u8;32]) -> Hash {
	    Hash(buffer)
	}

	pub fn to_hex(&self) -> String{
		to_hex(self.0.to_vec())
	}

	pub fn zeros() -> Hash{
		ZEROS
	}

	pub fn random() -> Hash {
		Hash::new(rand::random())
	}

	// Most significant 64 bits
	pub fn to_u64( &self ) -> u64 {
		let mut u_64: [u8;8] = Default::default();
    	u_64.clone_from_slice(&self.0[..8]);
		unsafe { transmute::<[u8;8], u64>(u_64) }
	}
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl Writeable for Hash {
    fn write(&self, writer: &mut Writer) -> Result<(), Error>{
	    writer.write_fixed_size(&self.0)?;
	    Ok(())
	}
} 

impl Readable for Hash {
    fn read(reader: &mut Reader) -> Result<Hash, Error>{
    	let mut buffer = [0u8;32];
		reader.read_fixed_size(&mut buffer)?;
		Ok(Hash::new(buffer))
	}
}


pub trait Hashable:Writeable {
    fn hash(&self) -> Hash {
    	let mut writer = HashWriter::new();
    	self.write(&mut writer).unwrap();
    	writer.finalize()
    }
}

struct HashWriter{
	state : Blake2b
}

impl HashWriter {
	
	fn new() -> HashWriter {
		HashWriter {
			state: Blake2b::new(32),
		}
	}

    pub fn finalize(self) -> Hash {
    	let output = &mut [0u8;32];
		output.copy_from_slice(self.state.finalize().as_bytes());
		Hash::new(*output)
	}
}

impl Writer for HashWriter{
    fn write_fixed_size(&mut self, bytes: &[u8]) -> Result<(), Error>{
    	self.state.update(bytes.as_ref());
    	Ok(())
    }

    fn flush(&mut self) -> Result<(), Error>{
        // self.finalize();
        Ok(())
    }
}