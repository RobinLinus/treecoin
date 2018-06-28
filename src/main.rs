use std::sync::RwLock;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write, Error};
use std::{thread, time};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
extern crate rand;
use std::str;

mod network;
use network::peer::{ Peer, PeerChannel }; 
use network::network::Network; 
use network::message::{ Message, MessageHeader, Address, EmptyMessageBody, Readable, Writeable };
mod blockchain;
use blockchain::block::Block;
use blockchain::primitives::{ Hash, message_type };
mod miner;
use miner::Miner;

mod utils;

fn main() {
	let seed_nodes = vec![ String::from("127.0.0.1:7000") ];
	let mut network = Network::new(seed_nodes);
	let mut miner = Miner::new();
	let mut cycle_count = 0;
	
	loop {
		match miner.poll_new_block() {
		    Some(block) => network.broadcast(&Message::new(message_type::BLOCK, block)),
		    None => (),
		}

		match network.poll_new_message() {
		    Some(mut channel) => network_protocol(&mut network, &mut channel),
		    None => (),
		};

		network.poll_network_jobs();
		// sleep
		thread::sleep(time::Duration::new(0,1));
		// print node stats 
		// cycle_count +=1;
		// do not print every cycle 
		// if( self.cycle_count % 500000 ) != 0{ continue; };
		// println!("Stats: cycle_count: {:?},  peer_count: {}", self.cycle_count , self.network.peers_count());    
	}
}

fn network_protocol(network: &mut Network, channel: &mut PeerChannel){
	let message_type = channel.message_header.message_type;
	
	match message_type {
		message_type::VER => {
			// Message::new(message_type::VER_ACK, EmptyMessageBody).write(&mut connection);
			respond(message_type::VER_ACK, EmptyMessageBody, channel);
			println!("incomming peer");
		}
		message_type::VER_ACK => {
			println!("connected to server");
			// send our address 
			respond(message_type::ADDRESS,  network.server.address(), channel)
		}
		message_type::ADDRESS => {
			let mut address = {
				let mut peer = channel.peer.write().unwrap();
				let mut connection = &*peer.connection.write().unwrap();
				Address::read(&mut connection as &mut Read).unwrap()
			};
			println!("received address: {:?}", address);
			if(network.add_to_address_book(&mut address)){
				network.broadcast(&Message::new(message_type::ADDRESS, address));
			}
		}
	    message_type::BLOCK => {
	    	let mut peer = channel.peer.write().unwrap();
			let mut connection = &*peer.connection.write().unwrap();
	    	let block = Block::read(&mut connection as &mut Read).unwrap();
	    	println!("received Block: {:?}", block );
	    },
	    error_type => {
	    	let mut peer = channel.peer.write().unwrap();
			let mut connection = &*peer.connection.write().unwrap();
	    	// empty connection 
			println!("Unknown Message Type {:?} from {:?}: {:?}", error_type, connection.peer_addr().unwrap().to_string(), channel.message_header);
			empty_buffer(&mut connection);
	    },
	}
}

fn respond<T:Writeable>(message_type: u32, body:T, channel: &mut PeerChannel){
	let mut peer = channel.peer.write().unwrap();
	let mut connection = &*peer.connection.write().unwrap();
	Message::new(message_type, body).write(&mut connection);
}

fn empty_buffer(connection:&mut Read){
	let mut buf = Vec::new();
	match connection.read_to_end(&mut buf) {
	    Ok(expr) => if(buf.len()>0){
					println!("Left in Buffer {:?}",buf);
			},
	    Err(_) => (),
	}
}
