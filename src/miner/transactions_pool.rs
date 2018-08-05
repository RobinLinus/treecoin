use blockchain::block::Block;
use blockchain::transaction::Transaction;
use std::collections::HashMap;
use blockchain::transaction::TransactionInput;

pub struct TransactionsPool {
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

        // add indexes for inputs
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
            // delete indexes for inputs 
            for input in &transaction.inputs {
                self.input_index.remove( &input );
            }
        }

        self.pool.remove( index ); 
    }


    pub fn pop( &mut self ) -> Option<Transaction> {
        match self.pool.pop() {
            Some(transaction) => {
                // delete index
                for input in &transaction.inputs {
                    self.input_index.remove( &input );
                }
                Some(transaction)
            },
            None => None,
        }
    }

    pub fn count( &self )-> usize {
        self.pool.len()
    }


    pub fn delete_spent_inputs(&mut self, block: &Block){
        for transaction in &block.transactions {
            for input in &transaction.inputs {
                self.delete_by_input(*input);
            }
        }
    }

}