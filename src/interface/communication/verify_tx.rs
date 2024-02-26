use crate::interface::UTXO;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifyTx {
    pub inputs: Vec<UTXO>,
    pub outputs: Vec<UTXO>,
    pub tx: String,
}
