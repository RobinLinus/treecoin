#[macro_use]
extern crate serde_derive;

mod utils;
mod protocol;
mod network;
mod blockchain;
mod miner;
mod archive;
mod wallet;

use archive::archive::start_archive;

use protocol::protocol::{ Protocol };
use protocol::protocol_config::ProtocolConfig;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let config_path = args[1].to_string(); 

	let genesis_block = ProtocolConfig::get_genesis_block();

	let config = ProtocolConfig::read_from_file( config_path ).unwrap();

	start_archive(config.get_archive_address(), config.archive_path.to_string());

	let mut protocol = Protocol::new( config, genesis_block );
	protocol.start();
}