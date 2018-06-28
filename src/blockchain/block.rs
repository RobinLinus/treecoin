use blockchain::transaction::Transaction;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::fmt;
use network::message::{Message,Writeable,Readable};
use std::marker::Sized;
use blockchain::primitives::{ Hash, message_type };


pub struct BlockHeader{
    prev: Hash,
    timestamp: u32
}

impl BlockHeader{
    pub fn new(prev: Hash, timestamp: u32) -> BlockHeader {
        BlockHeader{prev, timestamp}
    }

    pub fn to_message(self) -> Message<BlockHeader>{
        Message::new(message_type::BLOCK, self)
    }
}

impl Writeable for BlockHeader{
    fn write(&self, writer: &mut Write) -> Result<(), Error>{
        self.prev.write(writer);
        self.timestamp.write(writer)
    }
}

impl Readable for BlockHeader {
    fn read(reader: &mut Read) -> Result<BlockHeader, Error>{
        Ok(BlockHeader{
            prev: Hash::read(reader)?,
            timestamp: u32::read(reader)?
        })
    }
} 


#[derive(Debug)]
pub struct Block {
    header:BlockHeader,
    transactions:Vec<Transaction>
}

impl Block {
    pub fn new(header:BlockHeader) -> Block{
        Block{
            header,
            transactions : vec![]
        }
    }

    pub fn add_transaction(&mut self, transaction:Transaction){
        self.transactions.push(transaction);
    }
}

impl Writeable for Block {
    fn write(&self, writer: &mut Write) -> Result<(), Error>{
        
        // write header
        self.header.write(writer)?;

        // write transactions_count
        let transactions_count: u32 = self.transactions.len() as u32;

        // write all transactions 
        transactions_count.write(writer)?; 
        for transaction in &self.transactions {
            transaction.write(writer)?;
        }

        Ok(())
    }
} 

impl Readable for Block {
    fn read(reader: &mut Read) -> Result<Block, Error>{
        // read header
        let header = BlockHeader::read(reader)?;

        // read transactions_count
        let transactions_count: u32 = u32::read(reader)?;
        
        // read all transactions  
        let mut transactions = Vec::new();
        for transaction in 0..transactions_count {
            transactions.push(Transaction::read(reader)?);
        }
        
        Ok(Block{
            header,
            transactions
        })
    }
}



impl fmt::Debug for BlockHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BlockHeader\n\t{:?}\n\t{:?}", self.timestamp, self.prev)
    }
}

// impl fmt::Debug for Block {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}\n", self.header, self.prev)
//     }
// }
