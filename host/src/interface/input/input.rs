use crate::interface::{UTXO,PendingTransaction};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Inputs {
    pub state_t_1: String, // merkle root of the state at t+1 (all the blocks)
    pub state_t: String, // merkle root of the state at t (all the blocks)
    pub blocks_hash: Vec<String>, // hash of each block
} 