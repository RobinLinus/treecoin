use protocol::event::{ Event, EventResult,EventSource };
use blockchain::block::{ Block, BlockHeader };
use blockchain::transaction::{ Transaction, TransactionOutput, TransactionInput, Address, Signature, Value };
use blockchain::blockchain::Blockchain;
extern crate rand;
use self::rand::Rng;


pub struct Wallet;

impl Wallet{

	pub fn new() -> Wallet {
		Wallet {}
	}

	pub fn poll_new_transaction( &self, blockchain: &Blockchain ) -> EventResult{
		let random_value: u32 = rand::random();
		if( random_value > 10000000 ) { return Ok(Event::Nothing) }
		Ok( Event::Transaction(create_dummy_transaction() ))
	}
}


fn create_dummy_transaction() -> Transaction {

	let transaction_input_1 = TransactionInput{ block_id: rand::random(), transaction_id:rand::random() };
	let transaction_input_2 = TransactionInput{ block_id: rand::random(), transaction_id:rand::random() };
	let transaction_input_3 = TransactionInput{ block_id: rand::random(), transaction_id:rand::random() };


	let value_1: Value = rand::random();
	let balance_1: Value = rand::random();
	let address_1 : [u8;32] = rand::random();
	let address_1 = Address::new(address_1);
	let transaction_output_1 = TransactionOutput::new(address_1, value_1, balance_1);

	let value_2: Value = rand::random();
	let balance_2: Value = rand::random();
	let address_2 : [u8;32] = rand::random();
	let address_2 = Address::new(address_2);
	let transaction_output_2 = TransactionOutput::new(address_2, value_2, balance_2);

	let signature = Signature::new([255u8;64]);

	let inputs = vec![transaction_input_1,transaction_input_2,transaction_input_3];
	let outputs = vec![transaction_output_1,transaction_output_2];
	let mut transaction = Transaction::new(inputs, outputs);
	transaction.add_signature(signature);

	transaction
}