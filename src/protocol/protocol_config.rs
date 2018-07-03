extern crate serde;
extern crate serde_json;

use blockchain::transaction::Address;
use std::path::Path;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Clone , Eq, PartialEq, Deserialize, Hash)]
pub enum ServiceTypes {
	LiveNode,
	ArchiveNode
}

#[derive(Debug, Clone , Deserialize, Eq, PartialEq)]
pub struct Service{
	pub ip_address : String,
	pub port : u16
}

#[derive(Debug, Clone , Deserialize)]
pub struct ProtocolConfig {
	pub seed_nodes : Vec<String>,
	pub services : HashMap <ServiceTypes, Service>,
	pub archive_path : String,
	pub miner_address : String
}

impl ProtocolConfig {
	
	pub fn read_from_file<P: AsRef<Path>>(file_name: P) -> Result<ProtocolConfig, Box<Error>>{
		let file = File::open(file_name)?;
    	let config = serde_json::from_reader(file)?;
    	print!("Read config: {:?}", config);
    	Ok(config)
	}

	pub fn get_live_address(&self) -> String{
		let service = self.services.get(&ServiceTypes::LiveNode).unwrap();
		[ service.ip_address.to_string(), service.port.to_string() ].join(":")
	}

	pub fn get_archive_address(&self) -> String{
		let service = self.services.get(&ServiceTypes::ArchiveNode).unwrap();
		[ service.ip_address.to_string(), service.port.to_string() ].join(":")
	}

	pub fn get_miner_address(&self) -> Address {
		Address::from_hex(self.miner_address.to_string())
	}
}
