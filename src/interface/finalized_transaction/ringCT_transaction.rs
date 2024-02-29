use serde::{Deserialize, Serialize};

use crate::interface::PendingRingCT;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ringCTx {
    pub inputs: Vec<String>,  // hashes of the inputs UTXO
    pub outputs: Vec<String>, // hash of the outputs UTXO
    pub hash: String,         // hash of the transaction
}

impl ringCTx {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<ringCTx, Box<dyn std::error::Error>> {
        let tx: ringCTx = bincode::deserialize(bytes)?;
        Ok(tx)
    }
    pub fn from_pending_ringCTx(tx: PendingRingCT) -> ringCTx {

        let outputs: Vec<String> = tx.outputs.iter().map(|x| x.hash.clone()).collect();

        ringCTx {
            inputs: tx.inputs.clone(),
            outputs: outputs,
            hash: tx.hash.clone(),
        }
    }
}
