use crate::database::connection;
use crate::interface::block::Block;
use lmdb::Transaction as LmdbTransaction;
use lmdb::WriteFlags;
use chrono::{TimeZone, Utc};

pub fn insert_block(
    block_hash: String,
    block_number: u128,
    block: Block,
) -> Result<(), lmdb::Error> {
    let env = connection::create_or_open_env().unwrap();
    let block_db = connection::open_database(&env, Some("Blocks"))?;
    let index_db = connection::open_database(&env, Some("Index"))?;
    let binding_env = env;
    let block_data = block.to_bytes().unwrap();
    //store the block
    let mut txn1 = binding_env.begin_rw_txn()?;
    txn1.put(
        block_db,
        &block_hash.as_bytes(),
        &block_data,
        WriteFlags::empty(),
    )?;
    txn1.commit()?;

    //store the last block hash and number
    let mut txn2 = binding_env.begin_rw_txn()?;
    txn2.put(
        block_db,
        &"last_block_hash".to_string(),
        &block_hash.as_bytes(),
        WriteFlags::empty(),
    )?;
    txn2.commit()?;

    let mut txn3 = binding_env.begin_rw_txn()?;
    txn3.put(
        block_db,
        &"last_block_number".to_string(),
        &block_number.to_string().as_bytes(),
        WriteFlags::empty(),
    )?;
    txn3.commit()?;

    //store the index
    let mut txn4 = binding_env.begin_rw_txn()?;
    let binding_block_number = block_number.to_string();
    let key = binding_block_number.as_bytes();
    // display the block hash as hex string
    txn4.put(index_db, &key, &block_hash.as_bytes(), WriteFlags::empty())?;
    txn4.commit()?;

    // Convert the Unix timestamp to a DateTime<Utc>
    let datetime = Utc.timestamp(block.header.timestamp as i64 / 1000, 0);

    // Format the DateTime<Utc> to a string (optional)
    let date_string = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    println!(
        "<{:?}>\tNew Block: hash={:?}\tBlock height: {:?}\tTx count: {:?}",
        date_string,
        "0x".to_owned() + &block_hash,
        binding_block_number,
        block.transactions.len()
    );
    Ok(())
}

// this function deletes some blocks from the database
pub fn delete_blocks(indexes: Vec<String>) -> Result<(), lmdb::Error> {
    let env = connection::create_or_open_env().unwrap();
    let db_index = connection::open_database(&env, Some("Index"))?;
    let binding_env = env;
    let mut txn = binding_env.begin_rw_txn()?;
    // get the hash for each index and then delete the block
    let mut hashes: Vec<String> = [].to_vec();
    for index in indexes {
        let key = index.as_bytes();
        let block_hash = txn.get(db_index, &key).unwrap();
        let block_hash = String::from_utf8(block_hash.to_vec()).unwrap();
        hashes.push(block_hash);
        txn.del(db_index, &key, None)?;
    }
    txn.commit()?;

    let _ = delete_blocks_by_hashes(hashes)?;

    return Ok(());
}

// this function deletes blocks from the database by their hashes (should only be called from delete_blocks)
fn delete_blocks_by_hashes(hashes: Vec<String>) -> Result<(), lmdb::Error> {
    let env = connection::create_or_open_env().unwrap();
    let db_blocks = connection::open_database(&env, Some("Blocks"))?;
    let binding_env = env;
    let mut txn = binding_env.begin_rw_txn()?;
    for hash in hashes {
        txn.del(db_blocks, &hash.as_bytes(), None)?;
    }
    txn.commit()?;
    return Ok(());
}
