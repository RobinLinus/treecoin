use std::collections::HashMap;
use std::io;
use std::io::{Read, Write, Error};
use std::{thread, time};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
extern crate rand;
use std::str;


fn main() {
	let seed_nodes = vec![String::from("127.0.0.1:7000")];
	let network = Network::new(seed_nodes);
	network.start();
}

struct Network {
    peers: HashMap<String,Peer>,
    max_peers: usize,
    server: Server,
    address_book : Vec<String>,
    cycle_count: u64
}

impl Network {

	fn new(seed_nodes : Vec<String>) -> Network {
	    Network{
	    	peers: HashMap::new(),
	    	max_peers: 10,
	    	server: Server::new(),
	    	address_book : seed_nodes,
	    	cycle_count: 0
	    }
	}

	pub fn start(mut self){
		println!("Server listening on {:?}", self.server.address() );
		self.event_loop();
	}
    
    fn event_loop(mut self){
		println!("Event loop started!");
		loop {
			// send random messages for fun and testing 
			self.send_random_messages();

			// do we have a new message from a peer ?
			self.poll_messages();

			// can we connect to more peers? 
			self.connect_to_peers();

			// do we have a new incoming peer?
			self.poll_new_peers();

			self.printStats();
			
			// sleep
			self.sleep();
		}
	}

	fn poll_messages(&mut self)  {
	    for (address, mut peer) in &mut self.peers{
			match peer.read() {
			    Some(message) => {
			    	println!("Message from Peer: {}: '{}' ",  address,message);
			    	let command:String = message.chars().take(6).collect();
			    	let data:String = message.chars().skip(6).take(message.len()).collect();
			    	match command.as_ref() {
			    	    "hello?" => {
			    	    	self.address_book.push(data);
			    	    	peer.write(b"hello!");
			    	    },
			    	    "hello!" => peer.write(b"peers?"),
			    	    "peers?" => {
			    	    	peer.write([String::from("peers!"),self.address_book.join(" ")].join(" ").as_bytes())
			    	    },
			    	    "peers!" => {
			    	    	for address in data.split_whitespace(){
			    	    		self.address_book.push(address.to_string());
			    	    	}
			    	    },
			    	    _ => continue,
			    	}
			    },
			    None => continue,
			};
		}
	}

	fn poll_new_peers(&mut self){
		match self.server.poll_new_peer() {
		    Some(peer) => self.add_peer(peer),
		    None => return
		};
	}

	fn add_peer(&mut self, peer:Peer){
		self.peers.insert(peer.address(),peer);
	}

	fn connect_to_peers(&mut self){
		if (self.peers.len() > self.max_peers) { return; }
		// for each entry in our address book 
		for address in &mut (self.address_book){
			// is not yet connected? 
			if self.peers.contains_key(address) { continue ; }
			// is not ourselves? 
			if address.to_owned() == self.server.address() { continue; }
			// then connect  
			match self.server.connect_to_peer(address.to_string()) {
			    Some(mut peer) => {
			    	peer.write([String::from("hello?"),self.server.address()].join(" ").as_bytes());
			    	println!("connected to Peer: {}", peer.address() );
			    	self.peers.insert(peer.address(), peer);
			    	println!("new peer count: {}", self.peers.len() );
			    },
			    None => continue,
			};
		}
	}

	fn send_random_messages(&mut self){
		let random_value: u32 = rand::random();
		if( random_value > 10000 ) { return; }
		let hash = Hash::new(random_value.to_string());
		let block = Block::new(hash);
		for (address, mut peer) in &mut self.peers{
			peer.send(&block);
			println!("Sent Message '{:?}' to Peer: {:?}", random_value, address);
		}
	}

	fn sleep(&mut self){
		thread::sleep(std::time::Duration::new(0,200));
	}

	fn printStats(&mut self){
		self.cycle_count +=1;
		// do not print every cycle 
		if( self.cycle_count % 500000 ) != 0{ return };
		println!("Stats: cycle_count: {:?},  peer_count: {}", self.cycle_count , self.peers.len());
	}
}



 struct Server{
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
		Server{
			listener
		}
     }

     pub fn address(&self) -> String{
     	self.listener.local_addr().unwrap().to_string()
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
					Some(Peer::new(tcp_stream))
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


struct Peer {
    connection: TcpStream
}

impl Peer {
	pub fn new(connection:TcpStream)->Peer{
		connection.set_nonblocking(true).unwrap();
		Peer{connection}
	}

    pub fn read(&mut self) -> Option<String> {
    	let mut buffer = [0;512];
		match self.connection.read(&mut buffer) {
			Ok(0) => {
				// handle disconnect
				None
			}
		    Ok(_) => {
		    	let message = match String::from_utf8(buffer.to_vec()) {
		    	    Ok(msg) => msg,
		    	    Err(e) => String::from("Decoding Error"),
		    	};
		    	let message = message.trim_matches(char::from(0)).to_string();
				Some(message)
		    },
		    Err(_) => None,
		}
    }

    pub fn write(&mut self, msg: &[u8]){
    	self.connection.write_all(msg);
    	self.connection.flush().unwrap();
    }

    pub fn send(&mut self, message: &Writeable){
    	message.write(&mut self.connection);
    }

    pub fn address(&self) -> String{
    	self.connection.peer_addr().unwrap().to_string()
    }
}






/// Trait that every type that can be serialized as binary must implement.
/// Writes directly to a Writer, a utility type thinly wrapping an
/// underlying Write implementation.
pub trait Writeable {
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
}


struct Hash(String);

impl Hash {
    pub fn new(string: String) -> Hash {
	    Hash(string)
	}
}

impl Writeable for Hash {
    fn write(&self, writer: &mut Write) -> Result<(), Error>{
	    writer.write_all(self.0.as_bytes());
	    Ok(())
	}
} 

impl Readable for Hash {
    fn read(reader: &mut Read) -> Result<Hash, Error>{
    	let mut buffer = [0;32];
		let string = reader.read(&mut buffer).unwrap().to_string();
		Ok(Hash::new(string))
	}
}


struct Block{
	prev: Hash
}

impl Block{
	pub fn new(prev: Hash) -> Block {
	    Block{prev}
	}
}

impl Writeable for Block{
	fn write(&self, writer: &mut Write) -> Result<(), Error>{
	    self.prev.write(writer)
	}
}

impl Readable for Block {
	fn read(reader: &mut Read) -> Result<Block, Error>{
		Ok(Block{
			prev: Hash::read(reader).unwrap()
		})
	}
} 
    