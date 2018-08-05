use blockchain::transaction::Value;
use std::collections::HashMap;
use miner::transactions_pool::TransactionsPool;
use blockchain::blockchain::Blockchain;
use protocol::event::{ EventResult, Event };
use blockchain::block::{ Block, BlockHeader };
use blockchain::transaction::{ Transaction, TransactionOutput, Address };
extern crate rand;

pub struct Miner {
	
	is_active: bool,
	transactions_pool : TransactionsPool,
    miner_address : Address

}

impl Miner {
	
	pub fn new( miner_address: Address ) -> Miner {

		Miner {
			is_active : true,
			transactions_pool : TransactionsPool::new(),
            miner_address : miner_address
		}
		
	}

    pub fn on_state_update( &mut self, block : &Block, _blockchain: &Blockchain ){

        // delete all spent inputs from transactions pool       
        self.transactions_pool.delete_spent_inputs(block);
    }

    pub fn poll_new_block( &mut self, blockchain: &Blockchain ) -> EventResult {
    	if !self.is_active { return Ok(Event::Nothing) }

    	// Simulate mining with a dummy 
    	let random_value: u32 = rand::random();
		if random_value > 5000000 { return Ok(Event::Nothing) }

		// build a block 
		let block = self.compose_block( blockchain );

        Ok(Event::BlockMined(block))
    }

    pub fn add_transaction_to_pool( &mut self, transaction : Transaction ){
    	self.transactions_pool.add( transaction );
    }

    fn compose_block( &mut self, blockchain: &Blockchain ) -> Block {

    	// create a dummy block
        let timestamp = 1234;
        let block_header = BlockHeader::new(blockchain.state_hash(), timestamp, blockchain.difficulty_target );

        // create a coinbase transaction to reward this miner
        let value = blockchain.current_reward(); 
        let mut coinbase_output = TransactionOutput::new( self.miner_address, value );
        coinbase_output.balance = value; // Todo: what if miner_address's balance is non-zero ? 
        let reward_transaction = Transaction::new_coinbase(coinbase_output);

        // create a new block
    	let mut block = Block::new( block_header , reward_transaction );
    	
        // if an address receives multiple outputs in a single block we need to aggregate them
        let mut state_cache = HashMap::new();
        // fill the block with transactions from transactions pool
        loop {
            match self.transactions_pool.pop() {
                Some( transaction ) => {
                    let transaction = self.prepare_transaction( transaction, blockchain, &mut state_cache );
                    block.add_transaction( transaction );
                },
                None => break,
            }
        }

        return block
    }

    fn prepare_transaction( &self, mut transaction: Transaction, blockchain: &Blockchain, state_cache : &mut HashMap<Address,Value> ) -> Transaction{
        
        // set resulting balance of every output
        for output in &mut transaction.outputs {
            
            let old_balance = match state_cache.get( &output.address ) {
                Some( balance ) => *balance,
                None => blockchain.unspent_outputs.get_balance_by_address( output.address ),
            };

            let new_balance = old_balance + output.value;
            state_cache.insert(output.address, new_balance);
            output.balance = new_balance;
        }

        transaction
    }

    pub fn pool_count(&self) -> usize { 
        self.transactions_pool.count()
    }

    // pub fn start(&mut self){
    // 	self.is_active = true
    // }

    // pub fn stop(&mut self){
    // 	self.is_active = false
    // }
}



// impl EventSource for Miner {
	
//     fn poll(&mut self) -> EventResult {
// 		match self.poll_new_block() {
// 		    Some(block) => Ok(Event::BlockMined(block)),
// 		    None => Ok(Event::Nothing),
// 		}
// 	}
// }
