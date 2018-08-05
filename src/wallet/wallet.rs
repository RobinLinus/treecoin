use protocol::protocol_config::ProtocolConfig;
use protocol::event::{ Event, EventResult };
use blockchain::transaction::{ Transaction, TransactionOutput, Address, Signature};
use blockchain::blockchain::Blockchain;
extern crate rand;


pub struct Wallet;

impl Wallet{

	pub fn new() -> Wallet {
		Wallet {}
	}

	pub fn poll_new_transaction( &self, blockchain: &Blockchain, config : &ProtocolConfig ) -> EventResult{
		let random_value: u32 = rand::random();
		if random_value > 10000000 { return Ok(Event::Nothing) }
		
		let option = blockchain.unspent_outputs.get_by_address( config.get_miner_address() );

		match option {
		    Some( (transaction_input, mut transaction_output) ) => {
		    	let recipient_value = 2;
				let recipient_address = Address::new(rand::random());
				let recipient_output = TransactionOutput::new(recipient_address, recipient_value);
				
				transaction_output.value = transaction_output.balance - recipient_value;

				let inputs = vec![transaction_input];
				let outputs = vec![recipient_output, transaction_output];
				
				let mut transaction = Transaction::new(inputs, outputs);
				
				let signature = Signature::new([255u8;64]);
				transaction.add_signature(signature);
				Ok( Event::Transaction( transaction ))
		    },
		    None => Ok( Event::Nothing ),
		}

		

	}
}

// fn create_dummy_transaction() -> Transaction {

// 	let transaction_input_1 = TransactionInput{ block_id: rand::random(), transaction_id:rand::random() };
// 	let transaction_input_2 = TransactionInput{ block_id: rand::random(), transaction_id:rand::random() };
// 	let transaction_input_3 = TransactionInput{ block_id: rand::random(), transaction_id:rand::random() };


// 	let value_1: Value = rand::random();
// 	let balance_1: Value = rand::random();
// 	let address_1 : [u8;32] = rand::random();
// 	let address_1 = Address::new(address_1);
// 	let transaction_output_1 = TransactionOutput::new(address_1, value_1, balance_1);

// 	let value_2: Value = rand::random();
// 	let balance_2: Value = rand::random();
// 	let address_2 : [u8;32] = rand::random();
// 	let address_2 = Address::new(address_2);
// 	let transaction_output_2 = TransactionOutput::new(address_2, value_2, balance_2);

// 	let signature = Signature::new([255u8;64]);

// 	let inputs = vec![transaction_input_1,transaction_input_2,transaction_input_3];
// 	let outputs = vec![transaction_output_1,transaction_output_2];
// 	let mut transaction = Transaction::new(inputs, outputs);
// 	transaction.add_signature(signature);

// 	transaction
// }