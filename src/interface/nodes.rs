use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Validator {
    pub node: Node,
    pub pubkey: String,
    pub last_block_hash: String,
    pub last_block_number: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub ip: String,
    pub port: String,
    pub status: String,
}

impl Node {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Node, Box<dyn std::error::Error>> {
        let encoded = bincode::deserialize(bytes)?;
        Ok(encoded)
    }
}

impl Validator {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Validator, Box<dyn std::error::Error>> {
        let encoded = bincode::deserialize(bytes)?;
        Ok(encoded)
    }
}
