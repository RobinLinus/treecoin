use blockchain::transaction::Transaction;
use wallet::wallet::Wallet;
use protocol::protocol_config::ProtocolConfig;
use blockchain::blockchain::Blockchain;
use network::message::{ EmptyMessageBody, Message };
use network::peer::{ PeerTracker, PeerChannel, PeerInfo, PeerAddress };
use network::network::Network;
use protocol::event::{ EventResult, Event, EventListener, EventSource };
use blockchain::block::Block;
use miner::miner::Miner;
use utils::serializer::{ Readable, Writeable, DiscWriter };
use std::{ thread, time };



impl EventListener for Protocol {

	fn on_event( &mut self, event: Event ) -> EventResult {
		// mapping of all events
		match event {
		    Event::IncommingPeer(peer_tracker) => self.on_incomming_peer(peer_tracker),
			Event::OutgoingPeer(peer_tracker) => self.on_outgoing_peer(peer_tracker),
			Event::BlockMined(block) => self.on_block_mined(block),
			Event::MessageHeader(peer_channel) => self.on_message_header(peer_channel),
			Event::Transaction(transaction) => self.on_transaction(transaction),
			_ => Ok(Event::Nothing)
		}
	}

}

pub struct Protocol {
    config: ProtocolConfig,
    blockchain : Blockchain,
    network : Network,
    miner : Miner,
    wallet: Wallet, 
    cycle_count: u64
}

pub mod message_type {
	pub const VER: 				u32 = 1;
	pub const VER_ACK: 			u32 = 2;
	pub const ADDRESS: 			u32 = 3;
	// pub const GET_BLOCKS: 		u32 = 4;
	// pub const INV: 				u32 = 5;
	pub const BLOCK: 			u32 = 6;
	pub const TRANSACTION: 		u32 = 7;
}


impl Protocol {

	fn on_message_header( &mut self, peer_channel: PeerChannel ) -> EventResult {
		let message_type = peer_channel.message_header.message_type;
		match message_type {

		    message_type::VER => self.on_peer_info_message(peer_channel),
		    message_type::VER_ACK => self.on_peer_info_acknowledged_message(peer_channel),
		    message_type::BLOCK => self.on_block_message(peer_channel),
		    message_type::ADDRESS => self.on_address_message(peer_channel),
		    message_type::TRANSACTION => self.on_transaction_message(peer_channel),
		    
		    t => {
		    	println!("message type not implemented {:?}", t );
		    	Ok(Event::Nothing)
		    }

		}
	}

	fn on_incomming_peer( &mut self, peer_tracker: PeerTracker ) -> EventResult {
		// send our info
		let peer = peer_tracker.read().unwrap();
		let mut conn = peer.connection.write().unwrap();
		let info = PeerInfo::new( self.network.server.address(), self.blockchain.size() );
		info.to_message().write(&mut *conn)?;
		println!(">> Incoming Peer {:?}", peer.address());

		// add peer
		self.network.add_peer(peer_tracker.clone());
		Ok(Event::Nothing)
	}
	 
	fn on_outgoing_peer( &mut self, peer_tracker: PeerTracker ) -> EventResult {
		// send our info
		let peer = peer_tracker.read().unwrap();
		let mut conn = peer.connection.write().unwrap();
		let info = PeerInfo::new( self.network.server.address(), self.blockchain.size() );
		info.to_message().write( &mut *conn )?;
		println!( ">> Outgoing Peer {:?}", peer.address() );

		// add peer
		self.network.add_peer(peer_tracker.clone());

		// broadcast our address
		self.network.server.address().to_message().write(&mut *conn)?;
		Ok(Event::Nothing)
	}

	fn on_peer_info_message( &mut self, channel: PeerChannel ) -> EventResult {
		let mut peer = channel.peer.write().unwrap();
		let peer_info = {
			let mut conn = peer.connection.write().unwrap();
			let peer_info = PeerInfo::read(&mut *conn)?;
			Message::new(message_type::VER_ACK, EmptyMessageBody).write(&mut *conn)?;
			peer_info
		};
		println!(">> Received: {:?}", peer_info);

		// Todo check correctness of peer info

		peer.set_address( peer_info.server_address.string );
		Ok(Event::Nothing)
	}

	fn on_peer_info_acknowledged_message( &mut self, channel: PeerChannel ) -> EventResult {
		let peer = channel.peer.write().unwrap();
		println!(">> Acknowledged: {:?}", peer.address());
		Ok(Event::Nothing)
	}

	fn on_address_message( &mut self, channel: PeerChannel ) -> EventResult {
		let mut address = {
			let mut peer = channel.peer.write().unwrap();
			let mut connection = peer.connection.write().unwrap();
			PeerAddress::read(&mut *connection)?
		};
		println!(">> Received: {:?}", address);
		
		// don't broadcast known addresses
		if !self.network.add_to_address_book(&mut address) { return Ok(Event::Nothing); }
		
		// don't re-broadcast our own address
		if address.string == self.network.server.address().string{ return Ok(Event::Nothing); }
		
		self.network.broadcast(&address.to_message())?;

		Ok(Event::Nothing)
	}

    fn on_block_message( &mut self, channel: PeerChannel ) -> EventResult {
		let peer = channel.peer.write().unwrap();
		let mut conn = peer.connection.write().unwrap();
		let mut block = Block::read(&mut *conn)?;
			    
    	println!(">> Received: {:?}", block);
    	match self.blockchain.verify_block(&mut block) {
    	    Ok( _ ) => (),
    	    Err(e) => println!("\nVerification Error: {:?}", e),
    	};
    	self.blockchain.apply_block(&mut block)?;
    	self.miner.on_block(&mut block);
    	block.write( &mut DiscWriter::block_writer(&self.config.archive_path, self.blockchain.size() ))?;

    	Ok(Event::Nothing)
	}

	fn on_block_mined( &mut self, mut block: Block ) -> EventResult {
		self.blockchain.apply_block( &mut block )?;
		self.miner.update_head( self.blockchain.root_hash(), self.blockchain.difficulty_target );
		block.write( &mut DiscWriter::block_writer( &self.config.archive_path, self.blockchain.size()))?;
		self.network.broadcast( &block.to_message() )?;
		Ok(Event::Nothing)
	}

	fn on_transaction_message(&mut self, channel:PeerChannel ) -> EventResult {
		let transaction = {
			let mut peer = channel.peer.write().unwrap();
			let mut conn = peer.connection.write().unwrap();
			Transaction::read(&mut *conn)?
		};
		// self.blockchain.verify_transaction( &mut transaction );
		self.on_transaction(transaction)?;
		Ok(Event::Nothing)
	}

	fn on_transaction(&mut self, transaction: Transaction) -> EventResult {
		let message = Message::new( message_type::TRANSACTION, transaction );
		self.network.broadcast( &message )?;
		self.miner.add_transaction_to_pool( message.get_body() );
		Ok(Event::Nothing)
	}
 
	// fn on_too_many_peers(&mut self){
	// 	unimplemented!()
	// }

	// fn on_peer_disconnect(&mut self){
	// 	unimplemented!()
	// }
	
	// fn on_too_few_peers(&mut self){
	// 	unimplemented!()
	// }

	// fn on_sync_start(&mut self){
		
	// }

	// fn on_sync_complete(&mut self){
	// 	self.miner.start();
	// }

	// boilerplate
	pub fn new(config: ProtocolConfig, genesis_block: Block) -> Protocol{
		let network = Network::new(&config);
		let blockchain = Blockchain::new(genesis_block);
		let mut miner = Miner::new(config.get_miner_address());
		miner.update_head(blockchain.root_hash(), blockchain.difficulty_target);
		Protocol{
			miner,
			network,
			blockchain,
			cycle_count: 0,
			config,
			wallet: Wallet::new()
		}
	} 

	pub fn poll(&mut self) -> EventResult {
		self.poll_miner()?;
		self.poll_network()?;
		self.poll_wallet()?;
		self.cycle_count += 1;
		self.log_stats();
		Ok(Event::Nothing)
	}

	fn poll_miner(&mut self) -> EventResult {
		match self.miner.poll() {
		    Ok(event) => self.on_event(event),
		    Err(err) => Err(err),
		}
	}

	fn poll_network(&mut self) -> EventResult {
		match self.network.poll() {
		    Ok(event) => self.on_event(event),
		    Err(err) => Err(err),
		}
	}

	fn poll_wallet(&mut self) -> EventResult {
		match self.wallet.poll_new_transaction(&self.blockchain)  {
		    Ok(event) => self.on_event(event),
		    Err(err) => Err(err),
		}
	}

	pub fn start(&mut self){
		loop {
		  	match self.poll() {
		  	    Ok( _ ) => (),
		  	    Err(e) => println!("\nError in Loop: \n{:?}\n\n", e),
		  	}
		  	// sleep
			thread::sleep( time::Duration::from_millis(10) );
		}
	}

	fn log_stats(&self){
		// do not print every cycle 
		if( self.cycle_count % 500 ) != 0{ return };
		println!("\n\nStats: \n\tcycle_count: {:?} \n\tconnections: {} peers\n\tchain_lenght: {:?} blocks\n\tstate_hash: {:?} \n\tUTXO set: {:?} UTXOs \n\ttx pool: {:?} TXs\n\n", 
			self.cycle_count, 
			self.network.peers_count(), 
			self.blockchain.size(), 
			self.blockchain.root_hash(),
			self.blockchain.unspent_outputs_count(),
			self.miner.pool_count(),
		);    
	}
}


