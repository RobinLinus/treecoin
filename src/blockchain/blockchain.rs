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
    unspent_outputs: UnspentOutputs,
    chain_head: Block,
    state_tree : StateTree,
    pub difficulty_target : u32
}

impl Blockchain {
    
    pub fn apply_block( &mut self, block: &mut Block ) -> EventResult {
        self.state_tree.insert(block.hash());
        for transaction in &block.transactions {
            self.apply_transaction(transaction)?;
        }
        Ok(Event::Nothing)
    }

    fn apply_transaction( &mut self, transaction: &Transaction ) -> EventResult {
        
        // remove spent outputs
        for input in &transaction.inputs {
            self.unspent_outputs.0.remove( &input );
        }

        // add unspent outputs
        let block_height = self.size();
        let mut i = 0;
        for output in &transaction.outputs {
            let ouput_id = TransactionInput{ block_id: block_height, transaction_id: i };
            if let Some(id) = self.unspent_outputs.get_id_by_address(output.address){
                self.unspent_outputs.0.remove( &id );
            }
            self.unspent_outputs.0.insert( ouput_id, *output );
            i += 1;
        }

        // println!("{:?}", self.unspent_outputs.0 );
        Ok(Event::Nothing)
    }

    pub fn verify_block( &mut self, block: &mut Block ) -> EventResult {
    	// verify block header
        self.verify_block_header(&mut block.header)?;

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
        let outputs_sum = transaction.sum_output_values();

        // verify: inputs_sum - outputs_sum > 0 ( no coins out-of-thin-air )
        if outputs_sum > inputs_sum  { return Err( Error::InvalidCoinSum ); }
    	
        let fees = inputs_sum - outputs_sum; 

    	// verify signature
    	transaction.signature.verify_multi_sig(input_keys)?;

    	Ok(Event::Nothing)
    }

    pub fn new( genesis_block: Block ) -> Blockchain {
        Blockchain {
            state_tree : StateTree::new(genesis_block.hash()),
            difficulty_target : genesis_block.header.difficulty_target,
            chain_head : genesis_block,
            unspent_outputs : UnspentOutputs(HashMap::new())
        }
    }

    pub fn root_hash( &self ) -> Hash {
        self.state_tree.root_hash
    }

    pub fn size( &self ) -> u32 {
        self.state_tree.head_id
    }

    pub fn unspent_outputs_count( &self ) -> usize {
        self.unspent_outputs.0.len()
    }
}

struct UnspentOutputs( HashMap<TransactionInput, TransactionOutput> );

impl UnspentOutputs{

    // pub fn get_by_address(&self, address: Address) -> Option<TransactionOutput>{
    //     for (_id, output) in &self.0{
    //         if output.address == address{
    //             return Some(*output)
    //         }
    //     }
    //     return None
    // }

    pub fn get_id_by_address(&self, address: Address) -> Option<TransactionInput>{
        for (id, output) in &self.0{
            if output.address == address{
                return Some(*id)
            }
        }
        return None
    }
}

