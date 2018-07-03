use std::collections::HashMap;
use protocol::event::{ EventResult, Event, EventSource };
use blockchain::block::{ Block, BlockHeader };
use blockchain::transaction::{ Transaction, TransactionOutput, TransactionInput, Address };
use utils::Hash;
extern crate rand;

pub struct Miner{
	state_hash: Hash,
	difficulty_target : u32,
	is_active: bool,
	transactions_pool : TransactionsPool,
    miner_address : Address
}

impl Miner {
	
	pub fn new( miner_address: Address ) -> Miner {

		Miner {
			state_hash:Hash::zeros(),
			difficulty_target: 0,
			is_active : true,
			transactions_pool : TransactionsPool::new(),
            miner_address : miner_address
		}
		
	}

	pub fn update_head( &mut self, state_hash: Hash, difficulty_target: u32 ){
		self.state_hash = state_hash;
		self.difficulty_target = difficulty_target;
	}
	
    pub fn poll_new_block(&mut self) -> Option<Block>{
    	if !self.is_active { return None }

    	// Simulate mining with a dummy 
    	let random_value: u32 = rand::random();
		if random_value > 5000000 { return None }

		// create a dummy block
		let timestamp = 1234;
		let block_header = BlockHeader::new( self.state_hash, timestamp, self.difficulty_target );

		let block = self.compose_block( block_header );

		Some( block )
    }

    pub fn add_transaction_to_pool( &mut self, transaction : Transaction ){
    	self.transactions_pool.add( transaction );
    }

    pub fn on_block( &mut self, block : &mut Block ){
        self.update_head(block.header.state, block.header.difficulty_target);
    	
    	// collect all spent inputs 
		let mut spend_inputs = Vec::new();
    	for transaction in &mut block.transactions {
			for input in &mut transaction.inputs {
				spend_inputs.push(input);
			}
    	}

        // delete the spent inputs
        for spend_input in &mut spend_inputs {
            self.transactions_pool.delete_by_input(**spend_input);
        }
    }

    fn compose_block( &mut self, block_header : BlockHeader ) -> Block {
    	
    	let mut block = Block::new( block_header );
    	
        self.add_miner_reward(&mut block);

        // fill the block with transactions from transactions pool
        loop {
            match self.transactions_pool.pop() {
                Some(transaction) => block.add_transaction(transaction),
                None => break,
            }
        }

        return block
    }

    fn add_miner_reward(&self, block: &mut Block){
        // add a coinbase transaction to reward this miner
        let value = 10; // Todo: block reward should decrease over time 
        let balance = value; // Todo: what if address is non-zero ? 
        let coinbase_output = TransactionOutput::new( self.miner_address, value, balance );
        let coinbase_transaction = Transaction::new_coinbase(coinbase_output);
        block.add_transaction(coinbase_transaction);
    }


    // pub fn start(&mut self){
    // 	self.is_active = true
    // }

    // pub fn stop(&mut self){
    // 	self.is_active = false
    // }

    pub fn pool_count(&self) -> usize { 
        self.transactions_pool.count()
    }
}


struct TransactionsPool {
    pool : Vec<Transaction>,
    input_index: HashMap<TransactionInput, usize>
}

impl TransactionsPool {

    pub fn new() -> TransactionsPool {
        TransactionsPool{
            pool : Vec::new(),
            input_index : HashMap::new()
        }
    }
    
    pub fn add( &mut self, mut transaction: Transaction ) {

        let index = self.pool.len();

        for input in &mut transaction.inputs {
            self.input_index.insert( *input, index );
        }

        self.pool.push( transaction );
    }

    pub fn delete_by_input( &mut self, input : TransactionInput ) {
        
        let index = match self.input_index.get( &input ) {
            Some(index) => *index,
            None => return,
        };

        {
            let transaction = self.pool.get(index).unwrap();
            // delete indexes 
            for input in &transaction.inputs {
                self.input_index.remove( &input );
            }
        }
        self.pool.remove( index ); 
    }


    pub fn pop(&mut self) -> Option<Transaction> {
        match self.pool.pop() {
            Some(transaction) => {
                // delete indexes 
                for input in &transaction.inputs {
                    self.input_index.remove( &input );
                }
                Some(transaction)
            },
            None => None,
        }
    }

    pub fn count( &self )-> usize{
        self.pool.len()
    }

}



impl EventSource for Miner {
	fn poll(&mut self) -> EventResult{
		match self.poll_new_block() {
		    Some(block) => Ok(Event::BlockMined(block)),
		    None => Ok(Event::Nothing),
		}
		
	}
}

