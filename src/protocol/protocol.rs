use blockchain::blockchain::Blockchain;
use network::message::{ EmptyMessageBody, Message, MessageHeader };

use network::peer::{ PeerTracker, PeerChannel, PeerInfo, PeerAddress };
use network::network::Network;
use protocol::event::{ EventResult, Event, EventListener, EventSource };
use std::io::{ Read, Error };
use std::net::TcpStream;
use blockchain::block::Block;
use miner::miner::Miner;
use utils::serializer::{Serializer, Reader, Writer, Readable, Writeable};
use utils::hash::Hashable;

impl EventListener for Protocol{

	fn on_event( &mut self, event: Event ) -> EventResult{
		// mapping of all events
		match event {
		    Event::IncommingPeer(peer_tracker) => self.on_incomming_peer(peer_tracker),
			Event::OutgoingPeer(peer_tracker) => self.on_outgoing_peer(peer_tracker),
			Event::BlockMined(mut block) => self.on_block_mined(block),
			Event::MessageHeader(peer_channel) => self.on_message_header(peer_channel),
			Event::Nothing => (),
			_ => ()
		};
		Ok(Event::Nothing)
	}
}

pub struct Protocol {
    miner : Miner,
    network : Network,
    blockchain : Blockchain,
    cycle_count: u64
}

pub mod message_type{
	pub const VER: 		u32 = 1;
	pub const VER_ACK: 	u32 = 2;
	pub const BLOCK: 	u32 = 3;
	pub const ADDRESS: 	u32 = 4;
}

impl Protocol {

	fn on_message_header( &mut self, peer_channel: PeerChannel) {
		let message_type = peer_channel.message_header.message_type;
		match message_type {
		    message_type::VER => self.on_peer_info(peer_channel),
		    message_type::VER_ACK => self.on_peer_info_acknowledged(peer_channel),
		    message_type::BLOCK => self.on_block(peer_channel),
		    message_type::ADDRESS => self.on_address(peer_channel),
		    t => println!("message type not implemented {:?}", t )
		};
	}

	fn on_too_few_peers(&mut self){
		unimplemented!()
	}

	fn on_incomming_peer( &mut self, peer_tracker: PeerTracker ){
		// send our info
		let mut peer = peer_tracker.read().unwrap();
		let mut conn = peer.connection.write().unwrap();
		let info = PeerInfo::new(self.network.server.address());
		info.to_message().write(&mut *conn);
		println!(">> Incoming Peer {:?}", peer.address());

		// add peer
		self.network.add_peer(peer_tracker.clone());
	}
	 
	fn on_outgoing_peer( &mut self, peer_tracker: PeerTracker ){
		// send our info
		let mut peer = peer_tracker.read().unwrap();
		let mut conn = peer.connection.write().unwrap();
		let info = PeerInfo::new(self.network.server.address());
		info.to_message().write(&mut *conn);
		println!(">> Outgoing Peer {:?}", peer.address());

		// add peer
		self.network.add_peer(peer_tracker.clone());

		// broadcast our address
		self.network.server.address().to_message().write(&mut *conn);
	}

	fn on_peer_info(&mut self, channel: PeerChannel){
		
		let mut peer = channel.peer.write().unwrap();
		let peer_info = {
			let mut conn = peer.connection.write().unwrap();
			let peer_info = PeerInfo::read(&mut *conn).unwrap();
			Message::new(message_type::VER_ACK, EmptyMessageBody).write(&mut *conn);
			peer_info
		};
		println!(">> Received: {:?}", peer_info);

		// Todo check correctness of peer info

		peer.set_address(peer_info.server_address.string);

	}

	fn on_peer_info_acknowledged(&mut self, channel: PeerChannel){
		
		let mut peer = channel.peer.write().unwrap();
		let mut conn = peer.connection.write().unwrap();

		println!(">> Acknowledged: {:?}", peer.address());
	}


    fn on_block(&mut self, channel:PeerChannel){
		let mut peer = channel.peer.write().unwrap();
		let mut conn = peer.connection.write().unwrap();
		match Block::read(&mut *conn) {
		    Ok(mut block) => {
		    	println!(">> Received: {:?}", block);
		    	self.blockchain.verify_block(&mut block);
		    	self.blockchain.apply_block(&mut block);
		    	self.miner.update_head(self.blockchain.root_hash(), self.blockchain.difficulty_target);
		    },
		    Err(e) => println!("Block Read Error {:?}", e),
		}
	}

	fn on_address(&mut self, channel:PeerChannel){
		let mut address = {
			let mut peer = channel.peer.write().unwrap();
			let mut connection = peer.connection.write().unwrap();
			PeerAddress::read(&mut *connection).unwrap()
		};
		println!(">> Received: {:?}", address);
		
		// don't broadcast known addresses
		if(!self.network.add_to_address_book(&mut address)) { return; }
		
		// don't re-broadcast our own address
		if(address.string == self.network.server.address().string){ return; }
		
		self.network.broadcast(&address.to_message());
		
	}

    fn on_block_mined(&mut self, mut block: Block){
		self.blockchain.apply_block(&mut block);
		self.miner.update_head(self.blockchain.root_hash(), self.blockchain.difficulty_target);
		self.network.broadcast(&block.to_message());
	}

	fn on_transaction(&mut self){
		unimplemented!()
	}

	fn on_message(&mut self){
		unimplemented!();
	}
 
	fn on_too_many_peers(&mut self){
		unimplemented!()
	}

	fn on_peer_disconnect(&mut self){
		unimplemented!()
	}
	

	// boilerplate
	pub fn new(network: Network, mut miner: Miner, mut blockchain: Blockchain) -> Protocol{
		miner.update_head(blockchain.root_hash(), blockchain.difficulty_target);
		Protocol{
			miner,
			network,
			blockchain,
			cycle_count: 0
		}
	} 

	pub fn poll(&mut self){
		self.poll_miner();
		self.poll_network();
		self.cycle_count += 1;
		self.log_stats();
	}

	fn poll_miner(&mut self)-> EventResult{
		match self.miner.poll() {
		    Ok(event) => self.on_event(event),
		    Err(err) => Err(err),
		}
	}

	fn poll_network(&mut self) -> EventResult{
		match self.network.poll() {
		    Ok(event) => self.on_event(event),
		    Err(err) => Err(err),
		}
	}

	fn log_stats(&self){
		// do not print every cycle 
		if( self.cycle_count % 50000 ) != 0{ return };
		println!("\n\nStats: \n\tcycle_count: {:?} \n\tpeer_count: {} \n\tchain_lenght: {:?} \n\tstate_hash: {:?}", self.cycle_count , self.network.peers_count(), self.blockchain.size(), self.blockchain.root_hash());    
	}
}
