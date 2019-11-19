use crate::*;
use crypto_hash::*;
use std::fmt;
use hex::*;
use crypto_hash::Algorithm::SHA256;

pub struct Block{
    pub index: u32, //where in the blockchain the block is at
    pub timeStamp: u128 , //time block is created
    pub hash: BlockHash, //hash for the block
    pub lastBlockHash: BlockHash, //hash of the previous block in blockchain
    pub nonce: u32, // this is for the proof of work when mining or doing transactions
    pub payload: String,

}
impl fmt::Debug for Block{ //needs to be before impl Block. not sure why TODO look into this
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f,"Block ~ Index: {} ~ time created: {} ~ last blocks hash: {} ~ nonce (POW): {} ~ payload: {}",
           &self.index, &self.timeStamp, &hex::encode(&self.lastBlockHash), &self.nonce, &self.payload)
    }

}

impl Block{
    pub fn new(index: u32, timeStamp: u128, lastBlockHash: BlockHash, nonce: u32, payload: String) -> Self{
        Block{
            index,
            timeStamp,
            hash: vec![0;32], // makes vector array of 32 zeros. will be used for SHA-256 hashing
            lastBlockHash,
            nonce,
            payload,
        }
    }
    pub fn bytes(&self) -> Vec<u8>{
        let mut b = vec![];
        b.extend(&u32_bits_to_4bytes(&self.index));
        b.extend(&u128_bits_to_16bytes(&self.timeStamp));
        b.extend(&self.lastBlockHash);
        b.extend(&u32_bits_to_4bytes(&self.nonce));
        b.extend(self.payload.as_bytes());
        b
    }
    pub fn hashable(&self) -> Vec<u8> {
        digest(SHA256, &self.bytes())
    }

}
