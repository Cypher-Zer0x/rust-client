use crate::interface::{PaymentUTXO, UTXO};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PendingRingCT {
    pub inputs: Vec<String>, // Ethereum address of the depositor
    pub outputs: Vec<PaymentUTXO>,
    pub hash: String,      // hash of the hash of the transaction
    pub signature: String, // signature of the transaction
    pub fee: String,       // fee of the transaction
}

impl PendingRingCT {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<PendingRingCT, Box<dyn std::error::Error>> {
        let tx: PendingRingCT = bincode::deserialize(bytes)?;
        Ok(tx)
    }
}

//G*hash(clef publique view)*r(alaeatoire connu par envoyeur)+clef publique spend
