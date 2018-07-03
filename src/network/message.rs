use utils::hash::Hashable;
use utils::serializer::{ Reader, Readable, Writer, Writeable };
use std::io::Error;
use protocol::protocol::message_type;


const MAGIC_BYTES : u16 = 4294967295;

#[derive(Debug)]
pub struct Message<T:Writeable> {
    header: MessageHeader,
    body: T
}

impl <T:Writeable>Message<T> {
    pub fn get_body(self) -> T {
        self.body
    }
}

impl <T:Writeable>Message<T> {
    pub fn new(message_type: MessageType, mut body: T) -> Message<T> {
        Message{
            header: MessageHeader::new( MAGIC_BYTES, message_type),
            body: body
        }
    }
}

impl <T:Writeable> Writeable for Message<T> {
    fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        self.header.write(writer)?;
        self.body.write(writer)?;
        writer.flush()
    }
}

impl <T:Writeable> Hashable for Message<T>{}

#[derive(Debug)]
pub struct MessageHeader{
    magic_bytes: u16,
    pub message_type: MessageType,
}

impl MessageHeader {
    pub fn new(magic_bytes:u16, message_type:MessageType) -> MessageHeader{
    	MessageHeader{
    		magic_bytes: magic_bytes,
    		message_type: message_type
    	}
    }
}

impl Writeable for MessageHeader {
    fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        self.magic_bytes.write(writer)?;
	    self.message_type.write(writer)?;
	    Ok(())
	}
} 
  
impl Readable for MessageHeader{
    fn read(reader: &mut Reader) -> Result<MessageHeader, Error>{
        Ok( MessageHeader{ 
            magic_bytes: u16::read(reader)?, 
            message_type: u32::read(reader)?
        })
    }

}  

pub type MessageType = u32;



#[derive(Debug)]
pub struct EmptyMessageBody;
impl EmptyMessageBody {
    pub fn new() -> EmptyMessageBody {
        EmptyMessageBody{}
    }
}

impl Writeable for EmptyMessageBody {
     fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        Ok(())
    }
}