use crate::interface::finalized_transaction::{ringCTx, UserDepositTx};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transaction {
    UserDeposit(UserDepositTx),
    RingCT(ringCTx),
}

impl Transaction {
    pub fn from_bytes(bytes: &[u8]) -> Result<Transaction, Box<dyn std::error::Error>> {
        let (variant, data) = bytes.split_first().ok_or("Empty bytes array")?;
        // println!("variant: {:?}", variant);
        let utxo = match variant {
            0 => bincode::deserialize::<Transaction>(data)?,
            1 => bincode::deserialize::<Transaction>(data)?,
            _ => return Err("Unknown UTXO variant".into()),
        };
        Ok(utxo)
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            &Transaction::UserDeposit(_) => bytes.push(0),
            &Transaction::RingCT(_) => bytes.push(1),
        }
        bytes.extend(bincode::serialize(self).unwrap());
        bytes
    }
    pub fn get_transaction_type(&self) -> String {
        match self {
            Transaction::UserDeposit(_) => "UserDeposit".to_string(),
            Transaction::RingCT(_) => "RingCT".to_string(),
        }
    }
    pub fn from_user_deposit_tx(tx: UserDepositTx) -> Transaction {
        Transaction::UserDeposit(tx)
    }
    pub fn from_ringCTx(tx: ringCTx) -> Transaction {
        Transaction::RingCT(tx)
    }

    pub fn get_hash(&self) -> String {
        match self {
            Transaction::UserDeposit(tx) => tx.hash.clone(),
            Transaction::RingCT(tx) => tx.hash.clone(),
        }
    }
}
