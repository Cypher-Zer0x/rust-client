use crate::interface::{CoinbaseUTXO, UTXO};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use web3::signing::keccak256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDepositEvent {
    pub txId: String,     // deposit tx hash from the network used to deposit the funds
    pub amount: String,   // Amount deposited in wei
    pub currency: String, // the currency
    pub root_block_number: u64, // Root block number of the deposit
    pub root_blockchain: String, //Ticker for the root blockchain
    pub public_key: String, // Key image of the deposit
    pub r_g: String,      // rG = G*r
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PendingUserDepositTx {
    pub txId: String, // Ethereum address of the depositor
    pub output: UTXO, // hash of the UTXO
    pub hash: String, // hash of the transaction
}

impl PendingUserDepositTx {
    pub async fn from_user_deposit_event(
        event: UserDepositEvent,
    ) -> Result<PendingUserDepositTx, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res = client
            .post("http://127.0.0.1:3001/api/generateCommitment")
            // Assuming the API expects a structured JSON object
            .json(&serde_json::json!({ "amount": event.amount }))
            .send()
            .await?;

        if res.status().is_success() {
            let verify_response = res.json::<Value>().await?;
            if let Some(commitment) = verify_response["commitment"].as_str() {
                let output: UTXO = UTXO::Coinbase(CoinbaseUTXO::new(
                    "0x01".to_string(),
                    "deposit".to_string(),
                    0, // because only one output
                    event.public_key,
                    None,
                    event.amount,
                    event.currency,
                    commitment.to_string(),
                    event.r_g,
                ));
                let bytes_output = output.to_bytes(); // Assuming this method exists and works as expected
                return Ok(PendingUserDepositTx {
                    txId: event.txId,
                    hash: hex::encode(keccak256(&bytes_output)).to_string(),
                    output,
                });
            }
        }
        Err("Failed to process deposit event".into())
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
