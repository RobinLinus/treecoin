use blockchain::transaction::Value;
use blockchain::transaction::Address;
use blockchain::block::BlockHeader;
use blockchain::transaction::TransactionInput;
use std::collections::HashMap;
use blockchain::transaction::Transaction;
use blockchain::state_tree::StateTree;
use protocol::event::EventResult;
use protocol::event::Event;
use protocol::event::Error;
use utils::Hash;
use utils::hash::Hashable;
use blockchain::block::Block;
use blockchain::transaction::TransactionOutput;

pub struct Blockchain {
    pub unspent_outputs: UnspentOutputs,
    pub difficulty_target : u32,
    state_tree : StateTree
}

impl Blockchain {

    pub fn new( genesis_block: &Block ) -> Blockchain {
        Blockchain {
            state_tree : StateTree::new(genesis_block.hash()),
            difficulty_target : genesis_block.header.difficulty_target,
            unspent_outputs : UnspentOutputs(HashMap::new())
        }
    }
    
    pub fn apply_block( &mut self, block: &mut Block ) -> EventResult {
        // insert block into state tree
        self.state_tree.insert(block.hash());

        // apply miner's reward
        self.apply_miner_reward(block)?;

        // apply transactions 
        let mut transaction_id = 1; // index starts at one because zero is the miner's reward
        for transaction in &block.transactions {
            self.apply_transaction(transaction, transaction_id)?;
            transaction_id += 1;
        }
        Ok(Event::Nothing)
    }

    fn apply_miner_reward( &mut self, block : &Block )-> EventResult{
        self.apply_outputs(&block.reward, 0)
    }

    fn apply_transaction( &mut self, transaction: &Transaction, transaction_id : u32 ) -> EventResult {
        // remove spent outputs
        for input in &transaction.inputs {
            self.unspent_outputs.0.remove( &input );
        }

        self.apply_outputs(transaction, transaction_id)
    }

    fn apply_outputs( &mut self, transaction: &Transaction, transaction_id : u32) -> EventResult {
        // add unspent outputs
        let block_height = self.block_count();
        let mut output_id = 0;
        for output in &transaction.outputs {
            let ouput_id = TransactionInput{ block_id: block_height, transaction_id: transaction_id, output_id : output_id };
            // we aggregate all outputs of an address into one 
            // therefore we have to delete all other outputs with the same address
            if let Some(id) = self.unspent_outputs.get_id_by_address(output.address){
                self.unspent_outputs.0.remove( &id );
            }
            self.unspent_outputs.0.insert( ouput_id, *output );
            output_id += 1;
        }
        Ok(Event::Nothing)

    }

    pub fn verify_block( &mut self, block: &mut Block ) -> EventResult {
    	
        // verify block header
        self.verify_block_header(&mut block.header)?;

        // verify miner's reward
        self.verify_miner_reward(&mut block.reward)?;

        // verify transactions
    	for transaction in &mut block.transactions{
    		self.verify_transaction(transaction)?;
    	}

    	Ok( Event::Nothing )
    }

    pub fn verify_block_header( &mut self, block_header: &mut BlockHeader ) -> EventResult {
    	// verify chain head extension
    	if block_header.state != self.state_tree.root_hash { return Err( Error::StateMissMatch ) }
    	// verify difficulty target
    	if block_header.difficulty_target != self.difficulty_target { return Err( Error::InvalidDifficulty ) }
    	// verify proof of work
    	block_header.verify_proof_of_work()
    }

    pub fn verify_miner_reward( &mut self, reward: &mut Transaction ) -> EventResult {
        if reward.sum_outputs() > self.current_reward() { return Err( Error::InvalidReward ) }
        Ok ( Event::Nothing )
    }

    pub fn verify_transaction(&mut self, transaction: &mut Transaction) -> EventResult {
    	
    	let mut inputs_sum = 0;
    	let mut input_keys = Vec::new();

    	// collect all inputs
    	for input in &mut transaction.inputs{
    		// verify: input is unspent
    		match self.unspent_outputs.0.get(input) {
    		    Some( mut transaction_output ) => {
    		    	input_keys.push(transaction_output.address);
    		    	inputs_sum += transaction_output.balance;
    		    },
    		    None => return Err(Error::InvalidInput),
    		}
    	}

    	// calculate fees
        let outputs_sum = transaction.sum_outputs();

        // verify: inputs_sum - outputs_sum > 0 ( no coins out-of-thin-air )
        if outputs_sum > inputs_sum  { return Err( Error::InvalidCoinSum ); }
    	
        // let fees = inputs_sum - outputs_sum; 

    	// verify signature
    	transaction.signature.verify_multi_sig(input_keys)?;

    	Ok(Event::Nothing)
    }

    pub fn state_hash(&self ) -> Hash {
        self.state_tree.root_hash
    }

    pub fn block_count(&self ) -> u32 {
        self.state_tree.head_id
    }

    pub fn current_reward( &self ) -> Value {
        10
    }
}

pub struct UnspentOutputs( HashMap<TransactionInput, TransactionOutput> );

impl UnspentOutputs{

    pub fn get_by_address(&self, address: Address) -> Option<( TransactionInput, TransactionOutput )>{
        for (id, output) in &self.0{
            if output.address == address {
                return Some((*id,*output))
            }
        }
        return None
    }

    pub fn get_id_by_address(&self, address: Address) -> Option<TransactionInput>{
        for (id, output) in &self.0{
            if output.address == address{
                return Some(*id)
            }
        }
        return None
    }

    pub fn get_balance_by_address( &self, address : Address ) -> Value {
        for ( _id, output) in &self.0{
            if output.address == address{
                return output.balance
            }
        }
        return 0
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn log(&self){
        println!("\nUnspentOutputs");
        for (id, output) in &self.0{
            println!("{:?}: {:?}" , id, output);
        }
        println!("\n\n");
    }
}

