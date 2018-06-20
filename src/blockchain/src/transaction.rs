use primitives::Address;
use primitives::Value;
use primitives::Bytes;
use primitives::Witness;
use primitives::Crypto;


pub struct TransactionInput(u64);

pub struct TransactionOutput {
    address: Address,
    value: Value
}

pub struct Transaction {
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
    witnesses: Vec<Witness>
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Transaction{
        Transaction{
            inputs:inputs,
            outputs:outputs,
            witnesses:vec![]
        }
    } 

    pub fn fee() -> Value {
        unimplemented!();
    }

    pub fn add_witness(&mut self, witness:Witness) {
        self.witnesses.push(witness)
    }

    pub fn validate_witnesses(&self) -> bool {
        let hash = self.serialize();
        let witnesses = &(self.witnesses);
        for witness in witnesses{
            if !witness.verify(hash) {
                return false
            }
        }
        true
    }

    pub fn sum_values(&self) -> Value {
        let mut sum = 0;
        for output in &self.outputs{
            sum += output.value;
        }
        sum
    }

    pub fn serialize(&self) -> &Bytes{
        b"Is Elon Musk Satoshi?"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_transaction() {
        
        let tx_in1 = TransactionInput(1337);
        let keys1 = Crypto::generate_keys();
        let private_key1 = keys1.private_key;
        let tx_out1 = TransactionOutput{
            address:keys1.public_key.to_address(),
            value: 5
        };
        
        
        let tx_in2 = TransactionInput(3333);
        let keys2 = Crypto::generate_keys();
        let private_key2 = keys2.private_key;
        let tx_out2 = TransactionOutput{
            address:keys2.public_key.to_address(),
            value: 7
        };
        
        let inputs = vec![tx_in1,tx_in2];
        let outputs = vec![tx_out1,tx_out2];

        let mut tx = Transaction::new(inputs, outputs);
        let witness1 = Witness::new(&private_key1, tx.serialize());
        let witness2 = Witness::new(&private_key2, tx.serialize());

        tx.add_witness(witness1);
        tx.add_witness(witness2);

        assert_eq!(tx.sum_values(),12);
        assert!(tx.validate_witnesses());

        let fake_id = b"fake";
        let witness3 = Witness::new(&private_key2, fake_id );
        tx.add_witness(witness3);
        assert_eq!(tx.validate_witnesses(),false);
    }
}