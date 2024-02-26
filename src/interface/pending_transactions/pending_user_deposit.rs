use crate::interface::{CoinbaseUTXO, UTXO};
use serde::{Deserialize, Serialize};
use web3::signing::keccak256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDepositEvent {
    pub txId: String,          // deposit tx hash from the network used to deposit the funds
    pub amount: String,          // Amount deposited in wei
    pub currency: String,        // the currency
    pub root_block_number: u64,  // Root block number of the deposit
    pub root_blockchain: String, //Ticker for the root blockchain
    pub public_key: String,      // Key image of the deposit
    pub r_g: String,             // rG = G*r
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PendingUserDepositTx {
    pub txId: String, // Ethereum address of the depositor
    pub output: UTXO,   // hash of the UTXO
    pub hash: String,   // hash of the transaction
}

impl PendingUserDepositTx {
    pub fn from_user_deposit_event(event: UserDepositEvent) -> PendingUserDepositTx {
        let output: UTXO = UTXO::Coinbase(CoinbaseUTXO::new(
            "0x01".to_string(),
            "deposit".to_string(),
            0, // because only one output
            event.public_key,
            None,
            event.amount,
            event.currency,
            "TODO".to_string(),
            event.r_g,
        ));
        let bytes_output = output.to_bytes();
        PendingUserDepositTx {

            txId: event.txId,
            hash: hex::encode(keccak256(&bytes_output)).to_string(),
            output,
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<PendingUserDepositTx, Box<dyn std::error::Error>> {
        let tx: PendingUserDepositTx = bincode::deserialize(bytes)?;
        Ok(tx)
    }
}

//G*hash(clef publique view)*r(alaeatoire connu par envoyeur)+clef publique spend
