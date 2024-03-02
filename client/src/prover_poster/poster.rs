use crate::database::read_blocks;
use crate::database::write_state;
use crate::NetworkConfig;
use bonsai_sdk::alpha as bonsai_sdk;
use bonsai_sdk::responses::SnarkReceipt;
use dotenv::dotenv;
use eth_merkle_tree::tree::MerkleTree;
use ethers::abi::Abi;
use ethers::core::types::U256;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use host::{run_bonsai, run_stark2snark};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::error::Error;
use std::fs;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inputs {
    pub state_t_1: String, // merkle root of the state at t+1 (all the blocks)
    pub state_t: String,   // merkle root of the state at t (all the blocks)
    pub blocks_hash: Vec<String>, // hash of each block
}

impl Inputs {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

pub async fn verify_on_chain(
    snark_receipt: SnarkReceipt,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    //get the private key from the environment
    let private_key = std::env::var("PRIVATE_KEY")?;
    let contract_abi = r#"
    [{
        "inputs": [
          {
            "internalType": "uint256[2]",
            "name": "_pA",
            "type": "uint256[2]"
          },
          {
            "internalType": "uint256[2][2]",
            "name": "_pB",
            "type": "uint256[2][2]"
          },
          {
            "internalType": "uint256[2]",
            "name": "_pC",
            "type": "uint256[2]"
          },
          {
            "internalType": "uint256[4]",
            "name": "_pubSignals",
            "type": "uint256[4]"
          }
        ],
        "name": "publishProof",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      }
    ]
    "#;

    let config_data = fs::read_to_string("network.json").expect("Unable to read network.json");
    let networks: Vec<NetworkConfig> =
        serde_json::from_str(&config_data).expect("Error parsing JSON");

    for config in networks {
        let node_url_http = config.node_url_http;
        let contract_address = config.contract_address;
        let chain_id = config.chain_id;
        let wallet = Wallet::from_str(&private_key)?.with_chain_id(chain_id.parse::<u64>()?);
        let provider = Provider::<Http>::try_from(node_url_http)?;
        let provider = Arc::new(provider);
        let client = SignerMiddleware::new(provider.clone(), wallet);
        let client = Arc::new(client);
        let contract_address: H160 = contract_address.parse::<Address>()?;
        let contract_abi: Abi = serde_json::from_str(contract_abi)?;
        let contract = Contract::new(contract_address, contract_abi, client);
        let a0_bytes: [u8; 32] = snark_receipt.snark.a[0]
            .clone()
            .try_into()
            .expect("Incorrect length for a0");
        let a1_bytes: [u8; 32] = snark_receipt.snark.a[1]
            .clone()
            .try_into()
            .expect("Incorrect length for a1");
        let b0_bytes: [u8; 32] = snark_receipt.snark.b[0][0]
            .clone()
            .try_into()
            .expect("Incorrect length for b0");
        let b1_bytes: [u8; 32] = snark_receipt.snark.b[0][1]
            .clone()
            .try_into()
            .expect("Incorrect length for b1");
        let b2_bytes: [u8; 32] = snark_receipt.snark.b[1][0]
            .clone()
            .try_into()
            .expect("Incorrect length for b2");
        let b3_bytes: [u8; 32] = snark_receipt.snark.b[1][1]
            .clone()
            .try_into()
            .expect("Incorrect length for b3");
        let c0_bytes: [u8; 32] = snark_receipt.snark.c[0]
            .clone()
            .try_into()
            .expect("Incorrect length for c0");
        let c1_bytes: [u8; 32] = snark_receipt.snark.c[1]
            .clone()
            .try_into()
            .expect("Incorrect length for c1");
        let signal1_bytes: [u8; 32] = snark_receipt.snark.public[0]
            .clone()
            .try_into()
            .expect("Incorrect length for signal1");
        let signal2_bytes: [u8; 32] = snark_receipt.snark.public[1]
            .clone()
            .try_into()
            .expect("Incorrect length for signal2");
        let signal3_bytes: [u8; 32] = snark_receipt.snark.public[2]
            .clone()
            .try_into()
            .expect("Incorrect length for signal3");
        let signal4_bytes: [u8; 32] = snark_receipt.snark.public[3]
            .clone()
            .try_into()
            .expect("Incorrect length for signal4");

        let a0 = U256::from(a0_bytes);
        let a1 = U256::from(a1_bytes);
        let b0 = U256::from(b0_bytes);
        let b1 = U256::from(b1_bytes);
        let b2 = U256::from(b2_bytes);
        let b3 = U256::from(b3_bytes);
        let c0 = U256::from(c0_bytes);
        let c1 = U256::from(c1_bytes);
        let signal1 = U256::from(signal1_bytes);
        let signal2 = U256::from(signal2_bytes);
        let signal3 = U256::from(signal3_bytes);
        let signal4 = U256::from(signal4_bytes);
        if chain_id == "51" || chain_id == "1313161555" || chain_id == "23295" {
            let gas_price = provider.get_gas_price().await?;
            match contract
                .method::<(
                    [ethers::types::U256; 2],
                    [[ethers::types::U256; 2]; 2],
                    [ethers::types::U256; 2],
                    [ethers::types::U256; 4],
                ), ()>(
                    "publishProof",
                    (
                        [a0, a1],
                        [[b0, b1], [b2, b3]],
                        [c0, c1],
                        [signal1, signal2, signal3, signal4],
                    ),
                )?
                .legacy() // Use legacy transaction
                .gas_price(ethers::utils::parse_units(gas_price, "wei")?) // Specify gas price directly
                .send()
                .await
            {
                Ok(result) => println!(
                    "Published proof successfully on chain: {:?} at tx {}",
                    chain_id,
                    result.tx_hash()
                ),
                Err(e) => println!("Failed to call contract method: {:?}", e),
            }
        } else {
            match contract
                .method::<(
                    [ethers::types::U256; 2],
                    [[ethers::types::U256; 2]; 2],
                    [ethers::types::U256; 2],
                    [ethers::types::U256; 4],
                ), ()>(
                    "publishProof",
                    (
                        [a0, a1],
                        [[b0, b1], [b2, b3]],
                        [c0, c1],
                        [signal1, signal2, signal3, signal4],
                    ),
                )? // Use legacy transaction
                .send()
                .await
            {
                Ok(result) => println!(
                    "Published proof successfully on chain: {:?} at tx {}",
                    chain_id,
                    result.tx_hash()
                ),
                Err(e) => println!("Failed to call contract method: {:?}", e),
            }
        }
    }
    Ok(())
}

async fn prove_state_diff(
    state_t_1: String,
    state_t: String,
    blocks_hash: Vec<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Assuming `Inputs` and `to_bytes()` are defined elsewhere and are non-blocking
    let input = Inputs {
        state_t_1,
        state_t,
        blocks_hash,
    };
    let input_data = input.to_bytes();

    // Use spawn_blocking for potentially blocking operations
    let session_uuid = tokio::task::spawn_blocking(move || {
        run_bonsai(input_data).map_err(|e| e.to_string()) // Convert the error to String, which is Send + Sync
    })
    .await?;
    let snark_data = tokio::task::spawn_blocking(move || {
        run_stark2snark(session_uuid?).map_err(|e| e.to_string()) // Convert the error to String, which is Send + Sync
    })
    .await?;

    // Assuming verify_on_chain is properly async
    verify_on_chain(snark_data?)
        .await
        .expect("Failed to verify on chain");
    Ok(())
}

pub async fn run_prover() -> Result<(), Box<dyn Error>> {
    // Default to 0 for the last proven block if it's not found
    let last_block_proven = read_blocks::get_last_block_proven().unwrap_or(0);
    println!("Last block proven: {}", last_block_proven.clone());
    // Default to an empty string for the last state proven if it's not found
    let last_state_proven = read_blocks::get_last_state_proven().unwrap_or_default(); // `unwrap_or_default` defaults to an empty string for String type
    println!("Last state proven: {}", last_state_proven.clone());
    // Get the last block number, handling errors
    let last_block_number = match read_blocks::get_last_block_number() {
        Ok(Some(number)) => number,
        Ok(None) => {
            // It's expected that there might be no blocks at the very beginning, so handle accordingly
            // This could mean initializing the system or waiting for blocks to be created
            0 // Assuming 0 to indicate no blocks are yet available, adjust as needed
        }
        Err(e) => return Err(e.into()),
    };
    let mut state_data = vec![last_state_proven.clone()];
    let mut blocks_hash = Vec::new();
    // Even for the first run or when data is missing, proceed without error
    for block_num in last_block_proven..last_block_number {
        let block = match read_blocks::get_block_by_number(block_num) {
            Ok(block) => block,
            Err(e) => return Err(e.into()),
        };
        state_data.push(block.hash.clone());
        blocks_hash.push(block.hash);
    }
    // Compute the Merkle root
    let state_t_1 = get_merkle_root(state_data);
    // Call prove_state_diff with the necessary data
    if let Err(e) = prove_state_diff(state_t_1.clone(), last_state_proven, blocks_hash).await {
        // Handle errors from prove_state_diff, but continue the process
        return Err(e);
    }
    // If necessary, update the state in the database or perform additional steps here
    write_state::insert_last_state_proven(state_t_1.clone())?;
    write_state::insert_last_block_proven(last_block_number.clone().to_string())?;
    println!("Prover run completed successfully");
    println!("Last state proven: {}", state_t_1);
    println!("Last block proven: {}", last_block_number);
    Ok(())
}

fn get_merkle_root(hashes: Vec<String>) -> String {
    if hashes.is_empty() {
        return "".to_string();
    }
    let tree = MerkleTree::new(&hashes).expect("Tree creation error.");
    let root = tree.root.expect("Unable to access root");
    root.data
}
