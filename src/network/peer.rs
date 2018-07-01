
use utils::serializer::{Serializer, Reader, Writer, Readable, Writeable};
use std::io::{ Read, Write, Error };
use std::sync::{ Arc, RwLock };
use std::net::TcpStream;
use network::message::{ Message, MessageHeader, EmptyMessageBody };
use protocol::protocol::message_type;




pub type PeerTracker = Arc<RwLock<Peer>>;

pub type Connection = Arc<RwLock<Serializer>>;

pub struct PeerChannel {
    pub peer: PeerTracker,
    pub message_header: MessageHeader
}


pub struct Peer {
    pub connection: Connection,
    address: String
}

impl Peer {
	pub fn new(mut connection: TcpStream) -> Peer {
        connection.set_nonblocking(true);
		connection.set_nodelay(true);
		Peer{
            address: connection.peer_addr().unwrap().to_string(),
            connection: Arc::new(RwLock::new(Serializer{stream:connection})),
		}
	}

    pub fn send<T: Writeable>(&mut self, message: &Message<T>){
    	let mut connection = self.connection.write().unwrap();
    	message.write(&mut *connection);
    }
 
    pub fn receive(&mut self)->Option<MessageHeader>{
    	let mut connection = self.connection.write().unwrap();
    	
    	match MessageHeader::read(&mut *connection){
    		Ok(message_header) => Some(message_header),
    		Err( e ) => None
    	}
    }

    pub fn set_address(&mut self, address:String){
        self.address = address;
    }

    pub fn address(&self) -> String{
    	self.address.to_owned()
    }

    pub fn to_tracker(self) -> PeerTracker{
    	Arc::new(RwLock::new(self))
    }
}






#[derive(Debug)]
pub struct PeerAddress{
    pub string : String
}

impl PeerAddress{
    pub fn new(string: String)->PeerAddress{
        PeerAddress{string}
    }

    pub fn from_utf8(bytes:[u8;14]) -> PeerAddress{
        let v: Vec<u8> = bytes.iter().cloned().collect();
        PeerAddress::new(String::from_utf8(v).unwrap())
    }

    pub fn equals(&self, address:&String) -> bool{
        &self.string == address
    }

    pub fn to_message(self) -> Message<PeerAddress>{
        Message::new(message_type::ADDRESS, self)
    }
}

impl Writeable for PeerAddress{
     fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        let mut self_bytes =  self.string.as_bytes();
        let mut bytes = &mut [0u8;14];
        bytes.copy_from_slice(&self_bytes[0..14]);
        writer.write_fixed_size(bytes);
        Ok(())
    }
 }

 impl Readable for PeerAddress{
    fn read(reader: &mut Reader) -> Result<PeerAddress, Error>{
        let mut bytes = [0u8;14];
        match reader.read_fixed_size(&mut bytes) {
            Ok(expr) => {
                Ok(PeerAddress::from_utf8(bytes))
            },
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
pub struct PeerInfo {
    pub server_address: PeerAddress
}

impl PeerInfo {
    pub fn new(server_address: PeerAddress) -> PeerInfo{
        PeerInfo{
            server_address
        }
    }

    pub fn to_message(self) -> Message<PeerInfo>{
        Message::new(message_type::VER, self)
    }
}

impl Writeable for PeerInfo {
    fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        self.server_address.write(writer)?;
        Ok(())
    }
} 
  
impl Readable for PeerInfo{
    fn read(reader: &mut Reader) -> Result<PeerInfo, Error>{
        Ok( PeerInfo{ 
            server_address: PeerAddress::read(reader)?
        })
    }
}

