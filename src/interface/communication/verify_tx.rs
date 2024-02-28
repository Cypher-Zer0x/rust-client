use crate::interface::{PaymentUTXO, UTXO};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifyTx {
    pub inputs: Vec<String>,
    pub outputs: Vec<PaymentUTXO>,
    pub tx: String,
}
