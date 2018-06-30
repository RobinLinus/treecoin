use utils::serializer::{ Reader, Readable, Writer, Writeable };
use network::peer::{ Peer, PeerTracker, PeerChannel, PeerAddress };
use network::message::{ Message, MessageHeader, EmptyMessageBody };
use protocol::event::{ EventSource, EventResult, Event };
use protocol::protocol::message_type;
use std::sync::{ Arc, RwLock };
use std::collections::{ HashSet, HashMap };
use std::io;
use std::io::{ Read, Write, Error };
use std::{ thread, time };
use std::net::{ Shutdown, SocketAddr, TcpListener, TcpStream };
use std::str;


pub struct Network {
    peers: RwLock<HashMap<String, PeerTracker>>,
    peers_count_target: usize,
    pub server: Server,
    address_book : HashSet<String>
}

impl Network {

	pub fn new(seed_nodes : Vec<String>) -> Network {
	    Network{
	    	peers: RwLock::new(HashMap::new()),
	    	peers_count_target: 10,
	    	server: Server::new(),
	    	address_book : seed_nodes.iter().cloned().collect()
	    }
	}

	pub fn send_to_peer<T:Writeable>(&mut self, address:String, message: Message<T>)-> Result<(), Error>{
		let mut peers = self.peers.read().unwrap();
		peers.get(&address).unwrap().write().unwrap().send(&message);
		Ok(())
	}

	pub fn poll_new_message(&mut self) -> EventResult {
	    for (address, peer) in self.peers.read().unwrap().iter(){
			match peer.write().unwrap().receive() {
			    Some(message_header) => return Ok(Event::MessageHeader(
			    	PeerChannel{
			    		peer: peer.clone(),
			    		message_header: message_header
			    })),
			    None => continue,
			};
		}
		Ok(Event::Nothing)
	}

	fn poll_new_peers(&mut self) -> EventResult{
		match self.server.poll_new_peer() {
		    Some(peer) => {
		    	Ok(Event::IncommingPeer(peer.to_tracker()))
		    },
		    None => Ok(Event::Nothing)
		}
	}

	pub fn add_peer(&self, peer_tracker: PeerTracker){
		let mut peers = self.peers.write().unwrap();
		let address = peer_tracker.read().unwrap().address();
		peers.insert(address, peer_tracker);
	}

	fn connect_to_peers(&mut self) -> EventResult{

		if (self.peers.read().unwrap().len() > self.peers_count_target) { return Ok(Event::Nothing); }
		// for each entry in our address book 
		for address in &mut (self.address_book).iter(){
			// is not yet connected? 
			if self.is_connected(address.to_string()) { continue; }
			// is not ourselves? 
			if self.server.address().equals(&address.to_owned()) { continue; }
			// then connect  
			match self.server.connect_to_peer(address.to_string()) {
			    Some(mut peer) => {
			    	return Ok(Event::OutgoingPeer(peer.to_tracker()))
			    },
			    None => continue,
			};
		}
		return Ok(Event::Nothing);
	}

	fn is_connected(&self, address:String) -> bool{
		for (addr,peer_tracker) in self.peers.read().unwrap().iter(){
			if  peer_tracker.read().unwrap().address() == address { return true; }
		}
		false
	}

	pub fn broadcast<T:Writeable>(&mut self, message: &Message<T>){
		for (address, mut peer) in self.peers.read().unwrap().iter(){
			peer.write().unwrap().send(message);
			println!("Sent Message '{:?}' to Peer: {:?}", message, peer.read().unwrap().address());
		}
	}

	pub fn peers_count(&self) -> usize{
		self.peers.read().unwrap().len()
	}
	
	pub fn add_to_address_book(&mut self, address: &mut PeerAddress) -> bool{
		if(self.address_book.contains(&address.string)){ return false }
		self.address_book.insert(address.string.to_owned());
		return true
	}


}



impl EventSource for Network{
	fn poll(&mut self) -> EventResult {
		match self.poll_new_message()? {
		    Event::Nothing => (),
		    e => return Ok(e)
		};
		match self.connect_to_peers()? {
		    Event::Nothing => (),
		    e => return Ok(e)
		};
		match self.poll_new_peers()? {
		    Event::Nothing => (),
		    e => return Ok(e)
		};
		Ok(Event::Nothing)
	}
}



 pub struct Server{
 	listener: TcpListener
 }

 impl Server {
     fn new() -> Server {
     	let addrs = [
    		SocketAddr::from(([127,0,0,1], 7000)),
    		SocketAddr::from(([127,0,0,1], 7001)),
    		SocketAddr::from(([127,0,0,1], 7002)),
    		SocketAddr::from(([127,0,0,1], 7003)),
    		SocketAddr::from(([127,0,0,1], 7004)),
		];
     	let listener = TcpListener::bind(&addrs[..]).unwrap();
		listener.set_nonblocking(true).unwrap();
		let server = Server{ listener };
		println!("Server listening on {:?}", server.address() );
		server
     }

     pub fn address(&self) -> PeerAddress{
     	PeerAddress::new(self.listener.local_addr().unwrap().to_string())
     }

     pub fn connect_to_peer(&self, address:String) -> Option<Peer>{
     	match TcpStream::connect(address) {
     	    Ok(mut tcp_stream) => {
     			println!("connected to: {:?}", tcp_stream.peer_addr().unwrap());
     	    	Some(Peer::new(tcp_stream))
     	    },
     	    Err( _ ) => None,
     	}     	
     }
     
     pub fn poll_new_peer(&self) -> Option<Peer>{
     	match self.listener.accept() {
				Ok((mut tcp_stream, peer_addr)) => {
					println!("Peer connected! {}", peer_addr);
					Some( Peer::new(tcp_stream) )
				}
				Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
					None
				}
				Err(e) => {
					println!("Couldn't establish new client connection: {:?}", e);
					None
				}
			}
     }
 }
