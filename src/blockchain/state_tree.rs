use utils::Hash;
use std::io::Error;
use utils::hash::Hashable;
use utils::serializer::{ Writeable, Writer };

struct StateTree {
    root_hash: Hash,
    root_node: StateTreeNode,
    store: StateTreeStore
}

impl StateTree {
    
    pub fn insert(&mut self, hash: Hash){

    }

    pub fn revert(&mut self){

    }

}

struct StateTreeNode {
    left: Hash,
    right: Hash
}

impl StateTreeNode {
    
    fn has_right_child(&self) -> bool{
    	unimplemented!()
    }

    fn set_right_child(&mut self, hash:Hash){
    	self.right = hash;
    }
}

trait StateTreeStore {

	fn get(&self, hash: Hash) -> StateTreeNode;

	fn insert(&mut self, hash: Hash, node: StateTreeNode);

	fn delete(&mut self, hash: Hash);

}