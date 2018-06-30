use protocol::event::EventResult;
use utils::Hash;
use blockchain::block::Block;
use blockchain::transaction::TransactionOutput;

struct Blockchain {
    chain_head: Block,
    unspent_outputs: Vec<TransactionOutput>,
    state_tree : StateTree
}

impl Blockchain {
    pub fn apply_block(block: Block) -> EventResult {
        unimplemented!();
    }
}

struct StateTree{
	root: Hash
}