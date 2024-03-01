extern crate petgraph;

use hex;
use std::fmt::Display;
use tiny_keccak::{Hasher, Keccak};
#[derive(Debug)]
pub enum BytesError {
    ComparisonFailed(String, String),
    ConcatenateError(String, String),
    KeccakError(String),
}

fn keccak256(input: &str) -> Result<String, BytesError> {
    let mut value;
    if input.contains(",") {
        let inputs: Vec<&str> = input.split(",").collect();
        value = encode_packed(inputs[0].trim(), inputs[1].trim());
    } else {
        value = if input.starts_with("0x") {
            input[2..].to_string()
        } else {
            input.to_string()
        };

        if value.len() % 2 != 0 {
            value.insert(0, '0');
        }
    }
    let hash = match hex::decode(&value) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            return Err(BytesError::KeccakError(value));
        }
    };
    let mut digest = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(&*hash);
    hasher.finalize(&mut digest);
    let hex_string: Vec<String> = digest.iter().map(|b| format!("{:02x}", b)).collect();
    Ok(hex_string.concat())
}

impl Display for BytesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BytesError::ComparisonFailed(a, b) => {
                write!(f, "Comparison failed between {} and {}", a, b)
            }
            BytesError::ConcatenateError(a, b) => {
                write!(f, "Concatenate error between {} and {}", a, b)
            }
            BytesError::KeccakError(s) => write!(f, "Keccak error for {}", s),
        }
    }
}

impl std::error::Error for BytesError {}

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;

pub fn encode_packed(addr_str: &str, amount_str: &str) -> String {
    let addr_bytes = hex::decode(&addr_str[2..]).expect("Failed to decode address");
    let mut addr_padded = vec![0u8; 32];
    addr_padded[12..].copy_from_slice(&addr_bytes);
    let amount: u64 = amount_str.parse().expect("Failed to parse amount");
    let mut amount_bytes = [0u8; 32];
    for (i, byte) in amount.to_be_bytes().iter().rev().enumerate() {
        amount_bytes[31 - i] = *byte;
    }
    let mut packed = Vec::new();
    packed.extend_from_slice(&addr_padded);
    packed.extend_from_slice(&amount_bytes);
    packed.iter().map(|byte| format!("{:02x}", byte)).collect()
}

pub fn compare_bytes(a: &str, b: &str) -> Result<std::cmp::Ordering, hex::FromHexError> {
    let a_bytes = hex::decode(a)?;
    let b_bytes = hex::decode(b)?;

    Ok(a_bytes.cmp(&b_bytes))
}

pub fn concat_hex_strings(a: &str, b: &str) -> Result<String, hex::FromHexError> {
    let a_bytes = hex::decode(a)?;
    let b_bytes = hex::decode(b)?;

    let mut concatenated = a_bytes;
    concatenated.extend(b_bytes);

    Ok(hex::encode(concatenated))
}

pub fn hash_pair(a: &str, b: &str) -> Result<String, BytesError> {
    let sorted = match compare_bytes(a, b) {
        Ok(t) => {
            if t == std::cmp::Ordering::Greater {
                (b, a)
            } else {
                (a, b)
            }
        }
        Err(_) => {
            return Err(BytesError::ComparisonFailed(a.to_string(), b.to_string()));
        }
    };
    let concatenated = match concat_hex_strings(sorted.0, sorted.1) {
        Ok(t) => {
            format!("0x{}", t)
        }
        Err(_) => return Err(BytesError::ConcatenateError(a.to_string(), b.to_string())),
    };
    keccak256(concatenated.as_str())
}
/// Represents a node in the Merkle Tree.
pub struct MerkleNode {
    pub data: String,
}

/// Represents a Merkle Tree structure with its root and graph representation.
pub struct MerkleTree {
    pub root: Option<MerkleNode>,
    pub graph: DiGraph<String, ()>,
}

impl MerkleTree {
    /// Creates a new MerkleTree based on the provided data.
    ///
    /// # Errors
    ///
    /// - When there's a problem hashing the data with `keccak256`.
    /// - When the `hash_pair` function encounters issues.
    pub fn new(data: &Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut graph = DiGraph::new();
        let mut nodes: Vec<String> = data
            .into_iter()
            .map(|node| keccak256(node.as_str()).expect("Keccak Error."))
            .collect();
        let mut previous_layer_indices = Vec::new();

        for node_data in &nodes {
            let index = graph.add_node(node_data.clone());
            previous_layer_indices.push(index);
        }

        while nodes.len() > 1 {
            let mut new_level = Vec::new();
            let mut new_indices = Vec::new();

            for i in (0..nodes.len()).step_by(2) {
                let hashed_data: String = if i + 1 < nodes.len() {
                    hash_pair(nodes[i].as_str(), nodes[i + 1].as_str())?
                } else {
                    nodes[i].clone()
                };
                new_level.push(hashed_data.clone());
                let current_index = graph.add_node(hashed_data);
                if let Some(left_node_index) = previous_layer_indices.get(i) {
                    graph.add_edge(current_index, *left_node_index, ());
                }
                if let Some(right_node_index) = previous_layer_indices.get(i + 1) {
                    graph.add_edge(current_index, *right_node_index, ());
                }
                new_indices.push(current_index);
            }
            nodes = new_level;
            previous_layer_indices = new_indices;
        }

        let root_data = nodes[0].clone();
        let root_node = MerkleNode { data: root_data };

        Ok(MerkleTree {
            root: Some(root_node),
            graph,
        })
    }

    /// Locates the index of a specific leaf based on its hash.
    ///
    /// # Returns
    ///
    /// - `Some(index)` if the leaf with the specified hash is found.
    /// - `None` if the leaf with the specified hash is not found.
    pub fn locate_leaf(&self, target_hash: &String) -> Option<usize> {
        for (index, node_data) in self.graph.raw_nodes().iter().enumerate() {
            if &node_data.weight == target_hash {
                return Some(index);
            }
        }
        None
    }

    /// Generates a proof of inclusion for a specific leaf.
    ///
    /// # Panics
    ///
    /// - When the specified leaf index is out of bounds.
    ///
    /// # Returns
    ///
    /// - A vector containing hashes that make up the proof for the specified leaf.
    pub fn generate_proof(&self, leaf_index: usize) -> Vec<String> {
        let mut proof = Vec::new();
        let mut current_index = leaf_index;

        while let Some(parent_edge) = self
            .graph
            .edges_directed(NodeIndex::new(current_index), petgraph::Incoming)
            .next()
        {
            let parent = parent_edge.source();
            for edge in self.graph.edges_directed(parent, petgraph::Outgoing) {
                if edge.target().index() != current_index {
                    proof.push(format!("0x{}", self.graph[edge.target()].clone()));
                }
            }
            current_index = parent.index();
        }
        proof
    }
}
