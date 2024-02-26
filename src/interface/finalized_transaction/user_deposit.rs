use crate::interface::PendingUserDepositTx;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDepositTx {
    pub sender: String, // Ethereum address of the depositor
    pub output: String, // hash of the UTXO, not a vec because only one output
    pub hash: String,   // hash of the transaction
}

impl UserDepositTx {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<UserDepositTx, Box<dyn std::error::Error>> {
        let tx: UserDepositTx = bincode::deserialize(bytes)?;
        Ok(tx)
    }

    pub fn from_pending_user_deposit_tx(tx: PendingUserDepositTx) -> UserDepositTx {
        UserDepositTx {
            sender: tx.sender,
            hash: tx.hash,
            output: tx.output.get_hash(),
        }
    }
}

//G*hash(clef publique view)*r(alaeatoire connu par envoyeur)+clef publique spend
