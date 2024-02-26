use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockchainMetrics {
    pub number_of_block: u128,
    pub number_of_tx: u128,
    pub number_of_utxo: u128,
}

impl BlockchainMetrics {
    pub fn new(number_of_block: u128, number_of_tx: u128, number_of_utxo: u128) -> Self {
        BlockchainMetrics {
            number_of_block,
            number_of_tx,
            number_of_utxo,
        }
    }
}