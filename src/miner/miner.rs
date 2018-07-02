use protocol::event::{ EventResult, Event, EventSource };
use std::mem::transmute;
use blockchain::block::{ Block, BlockHeader };
use blockchain::transaction::{ Transaction, TransactionOutput, TransactionInput, Address, Signature, Value };
use utils::Hash;
extern crate rand;
use self::rand::Rng;

pub struct Miner{
	state_hash: Hash,
	difficulty_target : u32,
	is_active: bool
}

impl Miner {
	
	pub fn new() -> Miner{
		Miner {
			state_hash:Hash::zeros(),
			difficulty_target: 0,
			is_active : true
		}
	}

	pub fn update_head(&mut self, state_hash: Hash, difficulty_target: u32){
		self.state_hash = state_hash;
		self.difficulty_target = difficulty_target;
	}
	
    pub fn poll_new_block(&self) -> Option<Block>{
    	if !self.is_active { return None }

    	// Simulate mining with a dummy 
    	let random_value: u32 = rand::random();
		if( random_value > 5000000 ) { return None }

		let bytes: [u8; 4] = unsafe { transmute( random_value )};

		// create a dummy block
		let timestamp = 1234;
		let block_header = BlockHeader::new(self.state_hash, timestamp, self.difficulty_target );

		let mut block = Block::new(block_header);

		for i in 0..2 {
			block.add_transaction(create_dummy_transaction());
		}

		Some(block)
    }

    pub fn start(&mut self){
    	self.is_active = true
    }

    pub fn stop(&mut self){
    	self.is_active = false
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


fn create_dummy_transaction() -> Transaction{
	let transaction_input_1 = TransactionInput{ block_id: rand::random(), transaction_id:rand::random() };
	let transaction_input_2 = TransactionInput{ block_id: rand::random(), transaction_id:rand::random() };
	let transaction_input_3 = TransactionInput{ block_id: rand::random(), transaction_id:rand::random() };


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

	let signature = Signature::new([255u8;64]);

	let inputs = vec![transaction_input_1,transaction_input_2,transaction_input_3];
	let outputs = vec![transaction_output_1,transaction_output_2];
	let mut transaction = Transaction::new(inputs, outputs);
	transaction.add_signature(signature);

	transaction
}