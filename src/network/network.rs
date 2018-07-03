use protocol::protocol_config::ProtocolConfig;
use utils::serializer::{ Writeable };
use utils::hash::Hashable;
use network::peer::{ Peer, PeerTracker, PeerChannel, PeerAddress };
use network::message::{ Message };
use protocol::event::{ EventSource, EventResult, Event };
use std::sync::{ RwLock };
use std::collections::{ HashSet, HashMap };
use std::io;
use std::io::{ Error };
use std::net::{ TcpListener, TcpStream };



pub struct Network {
    peers: RwLock<HashMap<String, PeerTracker>>,
    peers_count_target: usize,
    pub server: Server,
    address_book : HashSet<String>,
    message_history : HashSet<u64>
}

impl Network {

	pub fn new(config: &ProtocolConfig) -> Network {
	    Network{
	    	peers: RwLock::new(HashMap::new()),
	    	peers_count_target: 10,
	    	server: Server::new( config.get_live_address() ),
	    	address_book : config.seed_nodes.iter().cloned().collect(),
	    	message_history : HashSet::new()
	    }
	}

	pub fn poll_new_message(&mut self) -> EventResult {
	    for ( _address, peer) in self.peers.read().unwrap().iter(){
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

		if self.peers.read().unwrap().len() > self.peers_count_target { return Ok(Event::Nothing); }
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
		for (_address, peer_tracker) in self.peers.read().unwrap().iter(){
			if  peer_tracker.read().unwrap().address() == address { return true; }
		}
		false
	}

	pub fn broadcast<T:Writeable>( &mut self, message: &Message<T> ){
		let hash = message.hash().to_u64();
		if !self.message_history.contains(&hash) {
	    	self.message_history.insert(hash);
			for (_address, mut peer) in self.peers.read().unwrap().iter(){
				peer.write().unwrap().send(message);
				// println!("Sent Message '{:?}' to Peer: {:?}", message, peer.read().unwrap().address());
			}
		}
	}

	pub fn peers_count(&self) -> usize{
		self.peers.read().unwrap().len()
	}
	
	pub fn add_to_address_book(&mut self, address: &mut PeerAddress) -> bool{
		if self.address_book.contains(&address.string) { return false }
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
 	
     fn new( socket_address: String ) -> Server {
     	let listener = TcpListener::bind(socket_address).unwrap();
		listener.set_nonblocking(true).unwrap();
		let server = Server{ listener };
		println!("Server listening on {:?}", server.address() );
		server
     }

     pub fn address(&self) -> PeerAddress {
     	PeerAddress::new( self.listener.local_addr().unwrap().to_string() )
     }

     pub fn connect_to_peer(&self, address:String) -> Option<Peer>{
     	match TcpStream::connect(address) {
     	    Ok(tcp_stream) => {
     			println!("Outgoing peer: {:?}", tcp_stream.peer_addr().unwrap() );
     	    	Some( Peer::new(tcp_stream) )
     	    },
     	    Err( _ ) => None,
     	}     	
     }
     
     pub fn poll_new_peer(&self) -> Option<Peer>{
     	match self.listener.accept() {
			Ok( (tcp_stream, peer_addr) ) => {
				println!("Incoming peer: {}", peer_addr);
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

