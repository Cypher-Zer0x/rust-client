use crate::interface::{PendingRingCT, PendingUserDepositTx};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PendingTransaction {
    PendingDeposit(PendingUserDepositTx),
    PendingRingCTx(PendingRingCT),
}

impl PendingTransaction {
    pub fn from_bytes(bytes: &[u8]) -> Result<PendingTransaction, Box<dyn std::error::Error>> {
        let tx: PendingTransaction = bincode::deserialize(bytes)?;
        Ok(tx)
    }
    pub fn get_transaction_type(&self) -> String {
        match self {
            PendingTransaction::PendingDeposit(_) => "UserDeposit".to_string(),
            PendingTransaction::PendingRingCTx(_) => "RingCT".to_string(),
        }
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }
    pub fn from_user_deposit_tx(tx: PendingUserDepositTx) -> PendingTransaction {
        PendingTransaction::PendingDeposit(tx)
    }

    pub fn get_hash(&self) -> String {
        match self {
            PendingTransaction::PendingDeposit(deposit) => deposit.hash.clone(),
            PendingTransaction::PendingRingCTx(ring) => ring.hash.clone(),
        }
    }
}
