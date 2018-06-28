use blockchain::primitives::message_type;
use std::sync::Arc;
use std::sync::RwLock;
use network::message::{ Message, MessageHeader, Address, EmptyMessageBody, Readable, Writeable };
use std::collections::{ HashSet, HashMap };
use std::io;
use std::io::{ Read, Write, Error };
use std::{ thread, time };
use std::net::{ Shutdown, SocketAddr, TcpListener, TcpStream };
extern crate rand;
use std::str;

use network::peer::{ Peer, PeerTracker, PeerChannel };

pub struct Network {
    peers: RwLock<HashMap<String, PeerTracker>>,
    max_peers: usize,
    pub server: Server,
    address_book : HashSet<String>
}

impl Network {

	pub fn new(seed_nodes : Vec<String>) -> Network {
	    Network{
	    	peers: RwLock::new(HashMap::new()),
	    	max_peers: 10,
	    	server: Server::new(),
	    	address_book : seed_nodes.iter().cloned().collect()
	    }
	}

	pub fn send_to_peer<T:Writeable>(&mut self, address:String, message: Message<T>)-> Result<(), Error>{
		let mut peers = self.peers.read().unwrap();
		peers.get(&address).unwrap().write().unwrap().send(&message);
		Ok(())
	}

	pub fn poll_new_message(&mut self) -> Option<PeerChannel> {
	    for (address, peer) in self.peers.read().unwrap().iter(){
			match peer.write().unwrap().receive() {
			    Some(message_header) => return Some( PeerChannel{
			    	peer: peer.clone(),
			    	message_header: message_header
			    }),
			    None => continue,
			};
		}
		None
	}

	fn poll_new_peers(&mut self){
		match self.server.poll_new_peer() {
		    Some(peer) => {
		    	self.add_peer(peer);
		    	self.server.address();
		    },
		    None => return
		};
	}

	fn add_peer(&self, peer: Peer){
		let mut peers = self.peers.write().unwrap();
		peers.insert(peer.address(), peer.to_tracker());
	}

    pub fn poll_network_jobs(&mut self){
		// do we have a new incoming peer?
		self.poll_new_peers();
		// can we connect to more peers? 
		self.connect_to_peers();
	}

	fn connect_to_peers(&mut self){

		if (self.peers.read().unwrap().len() > self.max_peers) { return; }
		// for each entry in our address book 
		for address in &mut (self.address_book).iter(){
			// is not yet connected? 
			if self.peers.read().unwrap().contains_key(address) { continue ; }
			// is not ourselves? 
			if self.server.address().equals(&address.to_owned()) { continue; }
			// then connect  
			match self.server.connect_to_peer(address.to_string()) {
			    Some(mut peer) => {
			    	self.peers.write().unwrap().insert(peer.address(), peer.to_tracker());
			    },
			    None => continue,
			};
		}
	}

	pub fn broadcast<T:Writeable>(&mut self, message: &Message<T>){
		for (address, mut peer) in self.peers.read().unwrap().iter(){
			peer.write().unwrap().send(message);
			println!("Sent Message '{:?}' to Peer: {:?}",message, peer.read().unwrap().address());
		}
	}

	pub fn peers_count(&self) -> usize{
		self.peers.read().unwrap().len()
	}
	
	pub fn add_to_address_book(&mut self, address: &mut Address) -> bool{
		if(self.address_book.contains(&address.string)){ return false }
		self.address_book.insert(address.string.to_owned());
		return true
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

     pub fn address(&self) -> Address{
     	Address::new(self.listener.local_addr().unwrap().to_string())
     }

     pub fn connect_to_peer(&self, address:String) -> Option<Peer>{
     	match TcpStream::connect(address) {
     	    Ok(tcp_stream) => {
     			println!("connected to: {:?}", tcp_stream.peer_addr().unwrap());
     	    	Some(Peer::new(tcp_stream))
     	    },
     	    Err( _ ) => None,
     	}     	
     }
     
     pub fn poll_new_peer(&self) -> Option<Peer>{
     	match self.listener.accept() {
				Ok((tcp_stream, peer_addr)) => {
					println!("Peer connected! {}", peer_addr);
					let peer = Peer::new(tcp_stream);
					Some( peer )
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


