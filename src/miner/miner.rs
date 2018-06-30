use protocol::event::{ EventResult, Event, EventSource };
use std::mem::transmute;
use blockchain::block::{ Block, BlockHeader };
use blockchain::transaction::{ Transaction, TransactionOutput, TransactionInput, Address, Signature, Value };
use utils::Hash;
extern crate rand;
use self::rand::Rng;

pub struct Miner;

impl Miner {
	pub fn new()->Miner{
		Miner{}
	}
	
    pub fn poll_new_block(&self) -> Option<Block>{

    	// Simulate mining with a dummy 
    	let random_value: u32 = rand::random();
		if( random_value > 10000 ) { return None }

		let bytes: [u8; 4] = unsafe { transmute( random_value )};

		// build a new block
		
		let timestamp = 1234;
		let block_header = BlockHeader::new(Hash::zeros(), timestamp);

		let mut block = Block::new(block_header);


		for i in 0..2 {
		    let transaction_input_1: TransactionInput = rand::random();
			let transaction_input_2: TransactionInput = rand::random();
			let transaction_input_3: TransactionInput = rand::random();

			let value_1: Value = rand::random();
			let balance_1: Value = rand::random();
			let address_1 : [u8;32] = rand::random();
			let address_1 = Address::new(address_1);
			let transaction_output_1 = TransactionOutput::new(address_1,value_1,balance_1);

			let value_2: Value = rand::random();
			let balance_2: Value = rand::random();
			let address_2 : [u8;32] = rand::random();
			let address_2 = Address::new(address_2);
			let transaction_output_2 = TransactionOutput::new(address_2,value_2,balance_2);

			let signature = Signature::new([0u8;64]);

			let inputs = vec![transaction_input_1,transaction_input_2,transaction_input_3];
			let outputs = vec![transaction_output_1,transaction_output_2];
			let mut transaction = Transaction::new(inputs, outputs);
			transaction.add_signature(signature);

			block.add_transaction(transaction);
		}

		Some(block)
    }
}

impl EventSource for Miner{
	fn poll(&mut self) -> EventResult{
		match self.poll_new_block() {
		    Some(block) => Ok(Event::BlockMined(block)),
		    None => Ok(Event::Nothing),
		}
		
	}
}