#[macro_use]
extern crate serde_derive;

mod utils;
mod protocol;
mod network;
mod blockchain;
mod miner;
mod archive;

use protocol::protocol::{ Protocol };
use protocol::protocol_config::ProtocolConfig;
use network::network::Network;
use blockchain::blockchain::Blockchain;
use blockchain::block::{ Block, BlockHeader };
use utils::Hash;
use miner::miner::Miner;
use std::{ thread, time };
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

	let config_path = args[1].to_string(); 
	// let config_path = String::from("src/config.json");

  let config = ProtocolConfig::read_from_file( config_path ).unwrap();

  archive::archive::start_archive(config.get_archive_address(), config.archive_path.to_string());
  let genesis_block = Block::new(BlockHeader::new(Hash::zeros(), 0, 8888));
  let mut protocol = Protocol::new(config, genesis_block);

  loop {
      protocol.poll();
      // sleep
  		thread::sleep( time::Duration::from_millis(10) );
  }

}