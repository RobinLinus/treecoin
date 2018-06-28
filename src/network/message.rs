use std::fmt::Debug;
use std::io::Write;

use std::io::Error;
use std::io::ErrorKind;

use std::io::Read;
use std::mem::transmute;

const MAGIC_BYTES :u32 = 123123;


/// Trait that every type that can be serialized as binary must implement.
/// Writes directly to a Writer, a utility type thinly wrapping an
/// underlying Write implementation.
pub trait Writeable : Debug{
	/// Write the data held by this Writeable to the provided writer
	fn write(&self, writer: &mut Write) -> Result<(), Error>;
}

/// Trait that every type that can be deserialized from binary must implement.
/// Reads directly to a Reader, a utility type thinly wrapping an
/// underlying Read implementation.
pub trait Readable
where
	Self: Sized,
{
	/// Reads the data necessary to this Readable from the provided reader
	fn read(reader: &mut Read) -> Result<Self, Error>;

    fn read_fixed_size(reader: &mut Read, buffer: &mut [u8] ) -> Result<(), Error>{
        match reader.read(buffer) {
            Ok(0) => Err(Error::new(ErrorKind::BrokenPipe, "disconnected")),
            Ok( _ ) => Ok(()),
            Err(e) => return Err(e),
        }
    }
}

#[derive(Debug)]
pub struct Message<T:Writeable> {
    header: MessageHeader,
    body: T
}

impl <T:Writeable>Message<T> {
    pub fn new(message_type: MessageType, body:T) -> Message<T> {
        Message{
            header: MessageHeader::new(MAGIC_BYTES,message_type),
            body: body
        }
    }
}

impl <T:Writeable> Writeable for Message<T> {
    fn write(&self, writer: &mut Write) -> Result<(), Error>{
        self.header.write(writer);
        self.body.write(writer)
    }
}

#[derive(Debug)]
pub struct MessageHeader{
    magic_bytes: u32,
    pub message_type: MessageType,
}

impl MessageHeader {
    pub fn new(magic_bytes:u32, message_type:MessageType) -> MessageHeader{
    	MessageHeader{
    		magic_bytes: magic_bytes,
    		message_type: message_type
    	}
    }
}

impl Writeable for MessageHeader {
    fn write(&self, writer: &mut Write) -> Result<(), Error>{
        self.magic_bytes.write(writer)?;
	    self.message_type.write(writer)?;
	    Ok(())
	}
} 
  
impl Readable for MessageHeader{
    fn read(reader: &mut Read) -> Result<MessageHeader, Error>{
        Ok( MessageHeader{ 
            magic_bytes: u32::read(reader)?, 
            message_type: u32::read(reader)?
        })
    }

}  

pub type MessageType = u32;


impl Writeable for u32{
     fn write(&self, writer: &mut Write) -> Result<(), Error>{
        let bytes = unsafe { transmute::<u32, [u8;4]>(*self) };
        writer.write(&bytes);
        Ok(())
    }
}

impl Readable for u32{
    fn read(reader: &mut Read) -> Result<u32, Error>{
        let mut bytes = [0u8;4];
        match Self::read_fixed_size(reader, &mut bytes) {
            Ok(expr) => {
                let u_32: u32 = unsafe { transmute::<[u8;4], u32>(bytes) };
                Ok(u_32)
            },
            Err(e) => Err(e),
        }
    }
}

impl Writeable for u64{
     fn write(&self, writer: &mut Write) -> Result<(), Error>{
        let bytes = unsafe { transmute::<u64, [u8;8]>(*self) };
        writer.write(&bytes);
        Ok(())
    }
}

impl Readable for u64{
    fn read(reader: &mut Read) -> Result<u64, Error>{
        let mut bytes = [0u8;8];
        match Self::read_fixed_size(reader, &mut bytes) {
            Ok(expr) => {
                let u_64: u64 = unsafe { transmute::<[u8;8], u64>(bytes) };
                Ok(u_64)
            },
            Err(e) => Err(e),
        }
    }
}


#[derive(Debug)]
pub struct EmptyMessageBody;
impl EmptyMessageBody {
    pub fn new() -> EmptyMessageBody {
        EmptyMessageBody{}
    }
}

impl Writeable for EmptyMessageBody {
     fn write(&self, writer: &mut Write) -> Result<(), Error>{
        Ok(())
    }
 }

#[derive(Debug)]
pub struct Address{
    pub string : String
}

impl Address{
    pub fn new(string: String)->Address{
        Address{string}
    }

    pub fn from_utf8(bytes:[u8;14]) -> Address{
        let v: Vec<u8> = bytes.iter().cloned().collect();
        Address::new(String::from_utf8(v).unwrap())
    }

    pub fn equals(&self, address:&String) -> bool{
        &self.string == address
    }
}

impl Writeable for Address{
     fn write(&self, writer: &mut Write) -> Result<(), Error>{
        let mut self_bytes =  self.string.as_bytes();
        let mut bytes = &mut [0u8;14];
        bytes.copy_from_slice(&self_bytes[0..14]);
        writer.write(bytes);
        Ok(())
    }
 }

 impl Readable for Address{
    fn read(reader: &mut Read) -> Result<Address, Error>{
        let mut bytes = [0u8;14];
        match Self::read_fixed_size(reader, &mut bytes) {
            Ok(expr) => {
                Ok(Address::from_utf8(bytes))
            },
            Err(e) => Err(e),
        }
    }
}