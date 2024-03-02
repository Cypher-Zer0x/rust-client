use crate::database::connection::{create_or_open_env, open_database};
use lmdb::Transaction as LmbdTransaction;

pub fn get_last_state_proven() -> Result<String, lmdb::Error> {
    let env = create_or_open_env().unwrap();
    let db = open_database(&env, Some("State"))?;
    let txn = env.begin_ro_txn()?;
    let key = "last_state_proven".as_bytes();
    match txn.get(db, &key) {
        Ok(value) => {
            let value_str = String::from_utf8(value.to_vec()).map_err(|_| lmdb::Error::NotFound)?;
            Ok(value_str)
        }
        Err(lmdb::Error::NotFound) => Ok("".to_string()), // Specifically handle not found as a valid case
        Err(e) => Err(e),                                 // Propagate other errors
    }
}

pub fn get_last_block_proven() -> Result<u128, lmdb::Error> {
    let env = create_or_open_env().unwrap();
    let db = open_database(&env, Some("State"))?;
    let txn = env.begin_ro_txn()?;
    let key = "last_block_proven".as_bytes();
    match txn.get(db, &key) {
        Ok(value) => {
            let value_str = String::from_utf8(value.to_vec()).map_err(|_| lmdb::Error::NotFound)?;
            let value = value_str
                .parse::<u128>()
                .map_err(|_| lmdb::Error::NotFound)?;
            Ok(value)
        }
        Err(lmdb::Error::NotFound) => Ok(0), // Specifically handle not found as a valid case
        Err(e) => Err(e),                    // Propagate other errors
    }
}
