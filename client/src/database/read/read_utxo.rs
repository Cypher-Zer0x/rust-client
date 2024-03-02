use crate::database::connection::{create_or_open_env, open_database};
use crate::interface::UTXO;
// use crate::interface::UTXO;
use lmdb::{Cursor, Transaction as LmbdTransaction};
//TODO MODIFY THIS SO WE USE THE UTXO ENUM, INSTEAD OF THE COINBASEUTXO STRUCT
// this function returns all the transactions in the mempool
pub fn get_utxo_set() -> Result<Vec<UTXO>, lmdb::Error> {
    let env = create_or_open_env().unwrap();
    let db = open_database(&env, Some("UTXO")).unwrap();
    let txn = env.begin_ro_txn()?;
    let mut data = Vec::new();
    {
        let mut cursor = txn.open_ro_cursor(db)?;
        for (_key, value) in cursor.iter() {
            let value = UTXO::from_bytes(value).unwrap();
            data.push(value);
        }
    }
    txn.commit()?;
    Ok(data)
}

pub fn get_utxo_by_hash(hash: String) -> Result<UTXO, lmdb::Error> {
    let env = create_or_open_env().unwrap();
    let db = open_database(&env, Some("UTXO")).unwrap();
    let txn = env.begin_ro_txn().unwrap();
    let key = hash.as_bytes();
    // println!("hash: {:?}", hash);
    let value = txn.get(db, &key).unwrap();
    let utxo = UTXO::from_bytes(value);
    Ok(utxo.unwrap())
}

pub fn get_number_UTXO() -> Result<u128, lmdb::Error> {
    // Attempt to create or open the environment, propagating any errors
    let env = create_or_open_env().unwrap();
    // Open the database, handling potential errors
    let db = open_database(&env, Some("UTXO"))?;
    // Begin a read-only transaction, handling errors
    let txn = env.begin_ro_txn()?;
    let mut count: u128 = 0;
    // Safely attempt to open a read-only cursor
    let mut cursor = txn.open_ro_cursor(db)?;
    // Iterate over items in the cursor, safely counting them
    for _ in cursor.iter() {
        count += 1;
    }
    // Ensure any resources are cleaned up here, if necessary, before returning
    Ok(count)
}
