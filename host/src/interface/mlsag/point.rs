use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Point {
    x: String,
    y: String,
}

impl Point {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Point, Box<dyn std::error::Error>> {
        let point: Point = bincode::deserialize(bytes)?;
        Ok(point)
    }
}
