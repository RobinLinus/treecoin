use std::collections::HashMap;
use std;
use utils::Hash;
use std::io::Error;
use utils::hash::Hashable;
use utils::serializer::{ Writeable, Writer };

pub struct StateTree {
    pub root_hash: Hash,
    pub head_id: u32,
    store: Box<StateTreeStore>
}

impl StateTree {
    
    pub fn new( genesis_hash: Hash ) -> StateTree {

        // create a root_node
        let mut root_node = StateTreeNode::new(1);
        // insert genesis hash into root_node
        root_node.left = genesis_hash;
        
        // create a store
        let mut store = DummyStore::new();
        
        // insert root_node into store 
        let root_hash = root_node.hash();
        store.insert(root_hash, root_node);

        // return a new StateTree instance
        StateTree {
            head_id : 0,
            root_hash : root_hash,
            store: Box::new(store)
        }
    }

    pub fn insert(&mut self, hash: Hash){
        
        // insert next hash to the right 
        let insert_id = self.head_id + 1;

        let mut curr_node = self.store.get(self.root_hash);
        let mut insert_path = Vec::new();
        
        // check if we need to extend the height
        if (is_power_of_two(insert_id)){
            curr_node = StateTreeNode::new(curr_node.height + 1);
            curr_node.left = self.root_hash;
        } else {
            self.store.delete(self.root_hash);
        }

        insert_path.push(curr_node);

        // traverse path down to current head 
        while curr_node.height > 1 {
            if get_bit_at(insert_id, curr_node.height - 1){
                // go right
                if(curr_node.right_is_empty()){
                    curr_node = StateTreeNode::new(curr_node.height - 1);
                } else {
                    curr_node = self.store.get_and_delete(curr_node.right);
                }
            } else {
                // go left 
                if(curr_node.left_is_empty()){
                    curr_node = StateTreeNode::new(curr_node.height - 1);
                } else {
                    curr_node = self.store.get_and_delete(curr_node.left);
                }
            }
            insert_path.push(curr_node);
        }
        
        // hash the new path from the leaf up to the root
        let mut curr_hash = hash;
        loop {
            match insert_path.pop() {
                Some(mut curr_node) => {
                    if get_bit_at(insert_id, curr_node.height - 1){
                        // go right
                        curr_node.right = curr_hash;
                    } else {
                        // go left
                        curr_node.left = curr_hash;
                    }
                    curr_hash = curr_node.hash();
                    self.store.insert(curr_hash, curr_node);
                },
                None => break,
            }
        }

        // update head 
        self.root_hash = curr_hash;
        self.head_id = insert_id;
    }

    pub fn revert(&mut self){
        
        let mut curr_node = self.store.get_and_delete(self.root_hash);
        let mut insert_path = Vec::new();
        insert_path.push(curr_node);

        let delete_id = self.head_id;

        // traverse down to head leaf
        while curr_node.height > 1 {
            if get_bit_at(delete_id, curr_node.height - 1){
                // go right
                curr_node = self.store.get_and_delete(curr_node.right);
            } else {
                // go left 
                curr_node = self.store.get_and_delete(curr_node.left);
            }
            insert_path.push(curr_node);
        }

        // hash the new path from the leaf up to the root
        let mut curr_hash = Hash::zeros();
        loop {
            match insert_path.pop() {
                Some(mut curr_node) => {
                    if get_bit_at(delete_id, curr_node.height - 1){
                        // go right
                        curr_node.right = curr_hash;
                    } else {
                        // go left
                        curr_node.left = curr_hash;
                    }
                    // insert only non-zero nodes
                    if (!curr_node.left_is_empty()) {
                        // check if we need to decrease height 
                        if insert_path.len() == 0 && curr_node.right_is_empty() {
                            curr_hash = curr_node.left;
                        } else {
                            curr_hash = curr_node.hash();
                            self.store.insert(curr_hash, curr_node);
                        }
                    }
                },
                None => break,
            }
        }

        // update head 
        self.root_hash = curr_hash;
        self.head_id = delete_id - 1 ;
    }

    pub fn root_node(&self)->StateTreeNode{
        self.store.get(self.root_hash)
    }
}


struct StateTreeNodeUpdater {
    parent: Box<StateTreeNodeUpdater>,
    node: StateTreeNode
}


fn is_power_of_two(input: u32) -> bool {
    let mut test = 1;
    loop {
        test = test * 2;
        if test > input { return false };
        if test == input { return true };
    }
}

fn get_bit_at(input: u32, index: u8) -> bool {
    if index < 32 {
        // let index = 31 - index; 
        input & (1 << index) != 0
    } else {
        false
    }
}


#[derive(Debug, Copy, Clone)]
struct StateTreeNode {
    left: Hash,
    right: Hash,
    height: u8
}

impl StateTreeNode {
    
    fn new(height:u8)-> StateTreeNode{
        StateTreeNode {
            height: height,
            left: Hash::zeros(),
            right: Hash::zeros(),
        }
    }

    fn left_is_empty(&self) -> bool{
        self.left == Hash::zeros()
    }

    fn right_is_empty(&self) -> bool{
        self.right == Hash::zeros()
    }
}

impl Hashable for StateTreeNode {}

impl Writeable for StateTreeNode {
    fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        self.left.write(writer)?;
        self.right.write(writer)?;
        self.height.write(writer)
    }
}

trait StateTreeStore {

	fn get(&self, hash: Hash) -> StateTreeNode;

    fn get_and_delete(&mut self, hash: Hash) -> StateTreeNode;

	fn insert(&mut self, hash: Hash, node: StateTreeNode);

	fn delete(&mut self, hash: Hash);

}



struct DummyStore(HashMap<Hash,StateTreeNode>);

impl DummyStore {
    pub fn new()->DummyStore{
        DummyStore(HashMap::new())
    }
}

impl StateTreeStore for DummyStore{

    fn get(&self, hash: Hash) -> StateTreeNode{
        *self.0.get(&hash).unwrap()
    }

    fn get_and_delete(&mut self, hash: Hash) -> StateTreeNode{
        let node = self.get(hash);
        self.delete(hash);
        node
    }

    fn insert(&mut self, hash: Hash, node: StateTreeNode){
        // println!("Insert Key: {:?}, Value: {:?} ", hash, node );
        self.0.insert(hash, node);
    }

    fn delete(&mut self, hash: Hash){
        // println!("Delete Key: {:?} ", hash );
        self.0.remove(&hash);
    }
} 


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn node() {

        assert_eq!(get_bit_at(1, 0), true);
        assert_eq!(get_bit_at(1, 1), false);
        assert_eq!(get_bit_at(1, 2), false);
        assert_eq!(get_bit_at(2, 0), false);
        assert_eq!(get_bit_at(2, 1), true);
        assert_eq!(get_bit_at(2, 2), false);
        assert_eq!(get_bit_at(2, 30), false);
        assert_eq!(get_bit_at(2, 31), false);
        assert_eq!(get_bit_at(2, 32), false);
    }
    #[test]
    fn insert() {
        let mut state_tree = StateTree::new(Hash::random());

        for i in [0u8;20].iter(){
            state_tree.insert(Hash::random());
        }
        let hash1 = state_tree.root_hash;
        println!("root_hash {:?}, head_id {:?}, \nroot_node: {:?}\n", state_tree.root_hash, state_tree.head_id, state_tree.root_node());

        for i in [0u8;80].iter(){
            state_tree.insert(Hash::random());
        }
        println!("root_hash {:?}, head_id {:?}, \nroot_node: {:?}\n", state_tree.root_hash, state_tree.head_id, state_tree.root_node());

        for i in [0u8;80].iter(){
            state_tree.revert();
        }
        let hash2 = state_tree.root_hash;
        println!("root_hash {:?}, head_id {:?}, \nroot_node: {:?}\n", state_tree.root_hash, state_tree.head_id, state_tree.root_node());
        
        assert_eq!(hash1, hash2);
    }
}


