use blockchain::block::BlockHeader;
use blockchain::transaction::TransactionInput;
use std::collections::HashMap;
use blockchain::transaction::Transaction;
use blockchain::state_tree::StateTree;
use protocol::event::EventResult;
use protocol::event::Event;
use utils::Hash;
use utils::hash::Hashable;
use blockchain::block::Block;
use blockchain::transaction::TransactionOutput;
use std;

pub struct Blockchain {
    unspent_outputs: HashMap<TransactionInput, TransactionOutput>,
    chain_head: Block,
    state_tree : StateTree,
    pub difficulty_target : u32
}

impl Blockchain {
    
    pub fn apply_block(&mut self, block: &mut Block) -> EventResult {
        self.state_tree.insert(block.hash());
        Ok(Event::Nothing)
    }

    pub fn verify_block(&mut self, block: &mut Block) -> EventResult {
    	self.verify_block_header(&mut block.header)?;

    	for transaction in &mut block.transactions{
    		self.verify_transaction(transaction)?;
    	}
    	Ok(Event::Nothing)
    }

    pub fn verify_block_header(&mut self, block_header: &mut BlockHeader) -> EventResult {
    	// verify chain head extension
    	if( block_header.state != self.state_tree.root_hash ) { return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "state miss-match")) }
    	// verify difficulty target
    	if( block_header.difficulty_target != self.difficulty_target ) { return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "difficulty_target miss-match")) }
    	// verify proof of work
    	block_header.verify_proof_of_work()?;

    	Ok(Event::Nothing)
    }

    pub fn verify_transaction(&mut self, transaction: &mut Transaction) -> EventResult {
    	
    	let mut inputs_sum = 0;
    	let mut input_keys = Vec::new();

    	// collect all inputs
    	for input in &mut transaction.inputs{
    		// verify: input is unspent
    		match self.unspent_outputs.get(input) {
    		    Some( mut transaction_output ) => {
    		    	input_keys.push(transaction_output.address);
    		    	inputs_sum += transaction_output.balance;
    		    },
    		    None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "transaction input doesn't exist!")),
    		}
    	}

    	// calculate fees
    	let fees = inputs_sum - transaction.sum_output_values();

    	// verify: inputs_sum - outputs_sum > 0 ( no coins out-of-thin-air )
    	if fees < 0 { return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "transaction creates coins!")); }

    	// verify signature
    	transaction.signature.verify_multi_sig(input_keys)?;

    	Ok(Event::Nothing)
    }

    pub fn new( mut genesis_block: Block ) -> Blockchain{
        Blockchain {
            state_tree : StateTree::new(genesis_block.hash()),
            difficulty_target : genesis_block.header.difficulty_target,
            chain_head : genesis_block,
            unspent_outputs : HashMap::new()
        }
    }

    pub fn root_hash( &self ) -> Hash {
        self.state_tree.root_hash
    }

    pub fn size( &self ) -> u32 {
        self.state_tree.head_id
    }
}