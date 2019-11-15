use crate::BlockHash;
use std::fmt;

pub struct Block{
    pub index: u32, //where in the blockchain the block is at
    pub timeStamp: u128 , //time block is created
    pub hash: BlockHash, //hash for the block
    pub lastBlockHash: BlockHash, //hash of the previous block in blockchain
    pub nonce: u32, // this is for the proof of work when mining or doing transactions

}
impl fmt::Debug for Block{ //needs to be before impl Block. not sure why TODO look into this
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f,"Block ~ Index: {} ~ time created: {}", self.index, self.timeStamp)
    }

}

impl Block{
    pub fn new(index: u32, timeStamp: u128, lastBlockHash: BlockHash, nonce: u32) -> Self{
        Block{
            index,
            timeStamp,
            hash: vec![0;32], // makes vector array of 32 zeros. will be used for SHA-256 hashing
            lastBlockHash,
            nonce
        }
    }



}