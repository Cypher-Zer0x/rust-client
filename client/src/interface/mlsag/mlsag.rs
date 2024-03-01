use crate::interface::Point;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct MLSAG {
    pub message: String,
    pub ring: Vec<Point>,
    pub c: String,
    pub responses: Vec<Vec<String>>,
    pub key_images: Vec<String>,
}

impl MLSAG {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<MLSAG, Box<dyn std::error::Error>> {
        let mlsag: MLSAG = bincode::deserialize(bytes)?;
        Ok(mlsag)
    }
}
