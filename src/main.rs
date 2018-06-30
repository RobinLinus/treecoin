mod utils;
mod protocol;
mod network;
mod blockchain;
mod miner;

use protocol::protocol::Protocol;
use network::network::Network;
use miner::miner::Miner;
use std::{thread, time};

fn main() {
	let seed_nodes = vec![ String::from("127.0.0.1:7000") ];
	let mut network = Network::new(seed_nodes);
	let mut miner = Miner::new();
    let mut protocol = Protocol::new(network,miner);
    loop {
        protocol.poll();
        // sleep
		thread::sleep(time::Duration::new(0,1));
    }
}