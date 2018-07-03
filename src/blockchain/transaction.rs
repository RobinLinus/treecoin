use utils::hash::Hashable;
use protocol::event::EventResult;
use protocol::event::Event;
pub use utils::serializer::{ Reader, Readable, Writer, Writeable };

use std::fmt;
use std::hash::Hash;
use std::io::{ Read, Write, Error };
use utils::hex;

pub type Value = u64;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct TransactionInput {
    pub block_id : u32,
    pub transaction_id : u32,
}

impl TransactionInput {
    pub fn new_coinbase() -> TransactionInput{
        TransactionInput{
            block_id : 0,
            transaction_id : 0
        }
    }
}

impl Readable for TransactionInput {
    fn read(reader: &mut Reader) -> Result<TransactionInput, Error>{
        Ok( TransactionInput {
                block_id : u32::read(reader)?,
                transaction_id : u32::read(reader)? 
            })
    }
}

impl Writeable for TransactionInput {
    fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        self.block_id.write(writer)?;
        self.transaction_id.write(writer)
    }
} 

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Address([u8;32]);

impl Address{

    pub fn new(bytes: [u8;32]) -> Address{
        Address(bytes)
    }

    pub fn from_hex(string : String) -> Address {
        let byte_vec = hex::from_hex( string ).unwrap();
        let mut bytes = [0u8;32];
        for (place, element) in bytes.iter_mut().zip(byte_vec.iter()) {
            *place = *element;
        }
        Address::new(bytes)
    }

    pub fn to_hex(&self) -> String{
        hex::to_hex( self.0.to_vec() )
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_hex())
    }
}

impl Writeable for Address {
    fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        writer.write_fixed_size(&self.0)?;
        Ok(())
    }
} 

impl Readable for Address {
    fn read(reader: &mut Reader) -> Result<Address, Error>{
        let mut buf = [0u8;32];
        reader.read_fixed_size(&mut buf)?;
        Ok(Address(buf))
    }
}


#[derive(Clone, Copy)]
pub struct TransactionOutput {
    pub address: Address,
    pub value: Value,
    pub balance: Value
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
    fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        self.address.write(writer)?;
        self.value.write(writer)?;
        self.balance.write(writer)?;
        Ok(())
    }
} 

impl Readable for TransactionOutput {
    fn read(reader: &mut Reader) -> Result<TransactionOutput, Error>{
        let address = Address::read(reader)?; 
        let value = Value::read(reader)?; 
        let balance = Value::read(reader)?; 
        Ok(TransactionOutput{address, value, balance})
    }
}

#[derive(Clone, Copy)]
pub struct Signature([u8;64]);

impl Signature{
    pub fn new(bytes: [u8;64]) -> Signature{
        Signature(bytes)
    }

    pub fn to_hex(&self) -> String{
        hex::to_hex(self.0.to_vec())
    }

    pub fn verify_multi_sig(&self, addresses: Vec<Address>)  -> EventResult {
        // Todo: implement multi signature verification...
        Ok(Event::Nothing)
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_hex())
    }
}

impl Writeable for Signature {
    fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        writer.write_fixed_size(&self.0)?;
        Ok(())
    }
} 

impl Readable for Signature {
    fn read(reader: &mut Reader) -> Result<Signature, Error>{
        let mut buf = [0u8;64];
        reader.read_fixed_size(&mut buf)?;
        Ok(Signature(buf))
    }
}


#[derive(Clone)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub signature: Signature
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Transaction{
        Transaction{
            inputs:inputs,
            outputs:outputs,
            signature: Signature([255u8; 64])
        }
    } 

    pub fn fee() -> Value {
        unimplemented!();
    }

    pub fn add_signature(&mut self, signature:Signature) {
        self.signature = signature;
    }

    pub fn sum_output_values(&self) -> Value {
        let mut sum = 0;
        for output in &self.outputs{
            sum += output.value;
        }
        sum
    }

    pub fn new_coinbase( output: TransactionOutput ) -> Transaction{
        let input = TransactionInput::new_coinbase();
        Transaction::new(vec![input], vec![output])
    }
}


impl Writeable for Transaction {
    fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        
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
    fn read(reader: &mut Reader) -> Result<Transaction, Error>{
        
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


impl Hashable for Transaction {}

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