use std::clone;

use crate::interface::finalized_transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockHeader {
    pub parent_block: String, // hash of the previous block
    pub block_number: u128,   // number of the block
    pub timestamp: u128,      // timestamp of the block creation
    pub merkle_root: String,  // merkle root of the transactions
}

#[derive(Debug, Serialize, Deserialize, clone::Clone)]
pub struct Block {
    pub header: BlockHeader,            // header of the block
    pub hash: String,                   // hash of the block
    pub transactions: Vec<Transaction>, // transactions included in the block
}

impl BlockHeader {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<BlockHeader, Box<dyn std::error::Error>> {
        let encoded = bincode::deserialize(bytes)?;
        Ok(encoded)
    }
}

impl Block {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Block, Box<dyn std::error::Error>> {
        let encoded = bincode::deserialize(bytes)?;
        Ok(encoded)
    }
}
