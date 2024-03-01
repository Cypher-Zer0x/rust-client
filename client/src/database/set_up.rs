use crate::{
    block_producer::block_producer::{get_merkle_root, get_timestamp},
    database::{insert_block, read_blocks::get_last_block_hash},
    interface::{Block, BlockHeader},
};
use lmdb::{DatabaseFlags, Environment};
use std::{fs, io, path::Path};
use web3::signing::keccak256;

/// Sets up the LMDB databases.
///
/// # Errors
///
/// Returns an error if the environment directory cannot be created, or if there
/// is a failure in setting up the LMDB environment or databases.
pub fn set_up_mldb() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("./lmdb_database");
    // More robust directory creation with error handling
    if !path.exists() {
        fs::create_dir_all(path).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to create LMDB environment directory: {}", e),
            )
        })?;
    }
    let env = Environment::new().set_max_dbs(7).open(path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to open LMDB environment: {}", e),
        )
    })?;
    // More robust database creation with error handling
    create_db(&env, "UTXO")?;
    create_db(&env, "Transactions")?;
    create_db(&env, "Blocks")?;
    create_db(&env, "Mempool")?;
    create_db(&env, "Index")?; // block indexes mapped to block hashes
    create_db(&env, "Validators")?;
    create_db(&env, "State")?;

    // if last block is none, insert genesis block in the database
    if let Ok(None) = get_last_block_hash() {
        let last_block_hash = "GENESIS".to_string();
        let block_number = 0;
        // we need to get the timestamp
        let timestamp = get_timestamp();
        // we need to create the merkle root
        let root = get_merkle_root(vec![]);
        // we create the block header
        let header = BlockHeader {
            parent_block: last_block_hash,
            block_number,
            timestamp,
            merkle_root: root,
        };
        let header_hash = hex::encode(keccak256(&header.to_bytes().unwrap()));
        let block = Block {
            header,
            transactions: vec![],
            hash: header_hash.clone(),
        };
        let _ = insert_block(header_hash, block_number, block);
    }
    println!("Set up completed successfully.");
    Ok(())
}

/// Helper function to create a database within the given environment.
///
/// # Parameters
///
/// * `env` - Reference to the LMDB `Environment`.
/// * `name` - Name of the database to create.
///
/// # Errors
///
/// Returns an error if the database cannot be created.
fn create_db(env: &Environment, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    env.create_db(Some(name), DatabaseFlags::empty())
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to create {} database: {}", name, e),
            )
        })?;
    Ok(())
}
