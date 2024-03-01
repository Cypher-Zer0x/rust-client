#![no_main]
use std::f64::consts::E;

// If you want to try std support, also update the guest Cargo.toml file
use risc0_zkvm::guest::env;
mod merkleTree;
risc0_zkvm::guest::entry!(main);
use crate::merkleTree::MerkleTree;
//mod interface;
//use interface::Inputs;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Inputs {
    pub state_t_1: String, // merkle root of the state at t+1 (all the blocks)
    pub state_t: String, // merkle root of the state at t (all the blocks)
    pub blocks_hash: Vec<String>, // hash of each block
} 

impl Inputs {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    
}
}
fn main() {
    // read the input
    let envVar: Vec<u8> = env::read::<Vec<u8>>();
    let input: Inputs = Inputs::from_bytes(&envVar);
    let binding_input = input.clone();
    println!("Input: {:?}", input);
    let mut combined = Vec::new();
    combined.push(input.state_t);
    combined.extend(input.blocks_hash);
    let mut tree = MerkleTree::new(&combined).expect("Failed to create Merkle Tree");
    let root = tree.root.expect("Failed to get root");
    println!("Root: {}", root.data);
    // TODO: do something with the input
    if input.state_t_1 == root.data {
        println!("Roots match");
        // write public output to the journal
        env::commit(&(binding_input.state_t_1,binding_input.state_t));
    } else {
        println!("Roots do not match");
    }
    
}
