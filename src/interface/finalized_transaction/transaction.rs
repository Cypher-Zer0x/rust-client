use crate::interface::finalized_transaction::{ringCTx, UserDepositTx};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transaction {
    UserDeposit(UserDepositTx),
    RingCT(ringCTx),
}

impl Transaction {
    pub fn from_bytes(bytes: &[u8]) -> Result<Transaction, Box<dyn std::error::Error>> {
        let tx: Transaction = bincode::deserialize(bytes)?;
        Ok(tx)
    }
    pub fn get_transaction_type(&self) -> String {
        match self {
            Transaction::UserDeposit(_) => "UserDeposit".to_string(),
            Transaction::RingCT(_) => "RingCT".to_string(),
        }
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }
    pub fn from_user_deposit_tx(tx: UserDepositTx) -> Transaction {
        Transaction::UserDeposit(tx)
    }
    pub fn from_ringCTx(tx: ringCTx) -> Transaction {
        Transaction::RingCT(tx)
    }
}
