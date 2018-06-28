use std::sync::Arc;
use std::sync::RwLock;
use std::net::TcpStream;
use network::message::{ Message, MessageHeader, Address, EmptyMessageBody, Readable, Writeable };
use blockchain::primitives::message_type;

pub struct Peer {
    pub connection: Connection
}

impl Peer {
	pub fn new(mut connection: TcpStream) -> Peer {
		Message::new(message_type::VER, EmptyMessageBody).write(&mut connection);
		connection.set_nonblocking(true).unwrap();
		Peer{
			connection: Arc::new(RwLock::new(connection))
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

    pub fn address(&self) -> String{
    	self.connection.read().unwrap().peer_addr().unwrap().to_string()
    }

    pub fn to_tracker(self) -> PeerTracker{
    	Arc::new(RwLock::new(self))
    }
}

pub type PeerTracker = Arc<RwLock<Peer>>;

pub type Connection = Arc<RwLock<TcpStream>>;

pub struct PeerChannel {
    pub peer: PeerTracker,
    pub message_header: MessageHeader
}

