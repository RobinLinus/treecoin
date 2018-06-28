use std::fmt;
use std::hash::Hash;
use std::io::Read;
use network::message::Writeable;
use utils::hex;
use std::io::Write;

use std::io::Error;

use network::message::Readable;

pub type Value = u64;

pub type TransactionInput = u64;

pub struct Address([u8;32]);

impl Address{
    pub fn new(bytes: [u8;32]) -> Address{
        Address(bytes)
    }
    pub fn to_hex(&self) -> String{
        hex::to_hex(self.0.to_vec())
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_hex())
    }
}

impl Writeable for Address {
    fn write(&self, writer: &mut Write) -> Result<(), Error>{
        writer.write(&self.0)?;
        Ok(())
    }
} 

impl Readable for Address {
    fn read(reader: &mut Read) -> Result<Address, Error>{
        let mut buf = [0u8;32];
        reader.read_exact(&mut buf)?;
        Ok(Address(buf))
    }
}



pub struct TransactionOutput {
    address: Address,
    value: Value,
    balance: Value
}

impl TransactionOutput {
    pub fn new(address:Address, value:Value, balance:Value)-> TransactionOutput{
        TransactionOutput{
            address, 
            value, 
            balance
        }
    }
}
impl Writeable for TransactionOutput {
    fn write(&self, writer: &mut Write) -> Result<(), Error>{
        self.address.write(writer)?;
        self.value.write(writer)?;
        self.balance.write(writer)?;
        Ok(())
    }
} 

impl Readable for TransactionOutput {
    fn read(reader: &mut Read) -> Result<TransactionOutput, Error>{
        let address = Address::read(reader)?; 
        let value = Value::read(reader)?; 
        let balance = Value::read(reader)?; 
        Ok(TransactionOutput{address, value, balance})
    }
}

pub struct Signature([u8;64]);

impl Signature{
    pub fn new(bytes: [u8;64]) -> Signature{
        Signature(bytes)
    }

    pub fn to_hex(&self) -> String{
        hex::to_hex(self.0.to_vec())
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_hex())
    }
}

impl Writeable for Signature {
    fn write(&self, writer: &mut Write) -> Result<(), Error>{
        writer.write(&self.0)?;
        Ok(())
    }
} 

impl Readable for Signature {
    fn read(reader: &mut Read) -> Result<Signature, Error>{
        let mut buf = [0u8;64];
        reader.read_exact(&mut buf)?;
        Ok(Signature(buf))
    }
}



pub struct Transaction {
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
    signature: Signature
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Transaction{
        Transaction{
            inputs:inputs,
            outputs:outputs,
            signature: Signature([0u8; 64])
        }
    } 

    pub fn fee() -> Value {
        unimplemented!();
    }

    pub fn add_signature(&mut self, signature:Signature) {
        // self.signaturees.push(signature)
    }

    pub fn sum_output_values(&self) -> Value {
        let mut sum = 0;
        for output in &self.outputs{
            sum += output.value;
        }
        sum
    }
}


impl Writeable for Transaction {
    fn write(&self, writer: &mut Write) -> Result<(), Error>{
        
        // write inputs_count
        let inputs_count: u32 = self.inputs.len() as u32;
        // write all inputs 
        inputs_count.write(writer)?; 
        for input in &self.inputs {
            input.write(writer)?;
        }

        // write outputs_count
        let outputs_count: u32 = self.outputs.len() as u32;
        // write all outputs
        outputs_count.write(writer)?; 
        for output in &self.outputs {
            output.write(writer)?;
        }

        // write signature
        self.signature.write(writer)?;

        Ok(())
    }
} 

impl Readable for Transaction {
    fn read(reader: &mut Read) -> Result<Transaction, Error>{
        
        // read inputs_count
        let inputs_count: u32 = u32::read(reader)?;
        
        // read all inputs  
        let mut inputs = Vec::new();
        for input in 0..inputs_count {
            inputs.push(TransactionInput::read(reader)?);
        }

        // read outputs_count
        let outputs_count: u32 = u32::read(reader)?;
        
        // read all outputs  
        let mut outputs = Vec::new();
        for output in 0..outputs_count {
            outputs.push(TransactionOutput::read(reader)?);
        }

        // read signature
        let signature = Signature::read(reader)?;
        
        Ok(Transaction{
            inputs,
            outputs,
            signature
        })
    }
}


impl fmt::Debug for TransactionOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nOutput:\n\t{:?}\n\t{:?}\t{:?}\n", self.address, self.value, self.balance)
    }
}

impl fmt::Debug for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nTransaction:\n\tInputs:{:?}\nOutputs:{:?}\nSignature:\n {:?}", self.inputs, self.outputs, self.signature)
    }
}