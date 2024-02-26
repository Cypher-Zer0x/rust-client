use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct LeftRigth {
    pub L: String,
    pub R: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct LigthRangeProof {
    pub V: String,
    pub A: String,
    pub S: String,
    pub T1: String,
    pub T2: String,
    pub tx: String,
    pub txbf: String,
    pub e: String,
    pub a0: String,
    pub b0: String,
    pub ind: Vec<LeftRigth>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct RangeProof {
    pub V: String,
    pub A: String,
    pub S: String,
    pub T1: String,
    pub T2: String,
    pub tx: String,
    pub txbf: String,
    pub e: String,
    pub a0: String,
    pub b0: String,
    pub ind: Vec<LeftRigth>,
    pub G: String,
    pub order: String,
}

impl LeftRigth {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<LeftRigth, Box<dyn std::error::Error>> {
        let utxo: LeftRigth = bincode::deserialize(bytes)?;
        Ok(utxo)
    }
}

impl LigthRangeProof {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<LigthRangeProof, Box<dyn std::error::Error>> {
        let utxo: LigthRangeProof = bincode::deserialize(bytes)?;
        Ok(utxo)
    }
}

impl RangeProof {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<RangeProof, Box<dyn std::error::Error>> {
        let utxo: RangeProof = bincode::deserialize(bytes)?;
        Ok(utxo)
    }
}
