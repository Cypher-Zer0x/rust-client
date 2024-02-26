use crate::database::connection::{create_or_open_env, open_database};
use crate::interface::Block;
use lmdb::{Cursor, Transaction as LmbdTransaction};

// this function is used to get the last block from db
pub fn get_last_block() -> Result<Option<Block>, lmdb::Error> {
    // get the last block hash
    let last_block_hash = get_last_block_hash();

    // get the last block data from the db
    let last_block = match last_block_hash {
        Ok(Some(last_block_hash)) => {
            let last_block = get_block_by_hash(last_block_hash);
            Some(last_block.unwrap())
        }
        Ok(None) => None,
        Err(e) => return Err(e),
    };

    Ok(last_block)
}

// this function is used by the block producer to get the last block hash, and use it to link the block to the previous one
pub fn get_last_block_hash() -> Result<Option<String>, lmdb::Error> {
    let env = create_or_open_env().unwrap();
    let db = open_database(&env, Some("Blocks"))?;
    let txn = env.begin_ro_txn()?;
    let key = "last_block_hash".as_bytes();
    match txn.get(db, &key) {
        Ok(value) => {
            let value_str = String::from_utf8(value.to_vec()).map_err(|_| lmdb::Error::NotFound)?;
            Ok(Some(value_str))
        }
        Err(lmdb::Error::NotFound) => Ok(None), // Specifically handle not found as a valid case
        Err(e) => Err(e),                       // Propagate other errors
    }
}

// this function is used by the block producer to get the last block number, and use it to link the block to the previous one
pub fn get_last_block_number() -> Result<Option<u128>, lmdb::Error> {
    let env = create_or_open_env().unwrap();
    let db = open_database(&env, Some("Blocks"))?;
    let txn = env.begin_ro_txn()?;
    let key = "last_block_number".as_bytes();
    match txn.get(db, &key) {
        Ok(value) => {
            let value_str = String::from_utf8(value.to_vec()).map_err(|_| lmdb::Error::NotFound)?;
            let value = value_str
                .parse::<u128>()
                .map_err(|_| lmdb::Error::NotFound)?;
            Ok(Some(value))
        }
        Err(lmdb::Error::NotFound) => Ok(Some(0)), // Specifically handle not found as a valid case
        Err(e) => Err(e),                          // Propagate other errors
    }
}

// this function returns all the blocks
pub fn get_blocks() -> Result<Vec<Block>, lmdb::Error> {
    let env = create_or_open_env().unwrap();
    let db = open_database(&env, Some("Blocks"))?;
    let txn = env.begin_ro_txn()?;
    let mut data: Vec<Block> = Vec::new();
    {
        let mut cursor = txn.open_ro_cursor(db)?;
        for (_key, value) in cursor.iter() {
            let key_str = std::str::from_utf8(_key).unwrap_or("");
            if key_str == "last_block_hash" {
                continue;
            }
            // Attempt to deserialize the value into a Block, logging errors without unwrapping
            match Block::from_bytes(value) {
                Ok(block) => data.push(block),
                Err(e) => eprintln!("Error deserializing block: {:?}", e),
            }
        }
    }
    txn.commit()?;
    Ok(data)
}

// this function returns a block by its hash
pub fn get_block_by_hash(block_hash: String) -> Result<Block, lmdb::Error> {
    let env = create_or_open_env().unwrap();
    let db = open_database(&env, Some("Blocks")).unwrap();
    let txn = env.begin_ro_txn().unwrap();
    let key = block_hash.as_bytes();
    let value = txn.get(db, &key).unwrap();
    let block = Block::from_bytes(value);
    Ok(block.unwrap())
}

// this function returns a block by its number
pub fn get_block_by_number(block_number: u128) -> Result<Block, lmdb::Error> {
    //first we get the hash by getting the into the index table
    println!("Getting block by number: {:?}", block_number);
    let env = create_or_open_env().unwrap();
    let db = open_database(&env, Some("Index"))?;
    let txn = env.begin_ro_txn().unwrap();
    let binding = block_number.to_string();
    let key = binding.as_bytes();
    println!("Index key: {:?}", &key);
    let value = txn.get(db, &key)?;
    let block_hash = String::from_utf8(value.to_vec()).unwrap();
    //then we get the block by getting the block by the hash
    let block = get_block_by_hash(block_hash);
    println!("{:?}", block);
    Ok(block.unwrap())
}

pub fn get_number_of_block() -> Result<u128, lmdb::Error> {
    // Attempt to create or open the environment, propagating any errors
    let env = create_or_open_env().unwrap();

    // Open the database, handling potential errors
    let db = open_database(&env, Some("Blocks"))?;

    // Begin a read-only transaction, handling errors
    let txn = env.begin_ro_txn()?;

    let mut count: u128 = 0;

    // Safely attempt to open a read-only cursor
    let mut cursor = txn.open_ro_cursor(db)?;

    // Iterate over items in the cursor, safely counting them
    for _ in cursor.iter() {
        count += 1;
    }
    // -2 because we have the last block hash and the last block number in the db
    Ok(count - 2)
}
