use crate::database::connection::{create_or_open_env, open_database};
use crate::interface::Transaction;
use lmdb::{Cursor, Transaction as LmbdTransaction};

// this function returns all the transactions that have been done
pub fn get_transactions() -> Result<Vec<Transaction>, lmdb::Error> {
    let env = create_or_open_env().unwrap();
    let db = open_database(&env, Some("Transactions")).unwrap();
    let txn = env.begin_ro_txn()?;
    let mut data = Vec::new();
    {
        let mut cursor = txn.open_ro_cursor(db)?;
        for (_key, value) in cursor.iter() {
            let value = Transaction::from_bytes(value);
            data.push(value.unwrap());
        }
    }
    txn.commit()?;
    Ok(data)
}

// this function returns a transaction by its hash
pub fn get_transaction_by_hash(tx_hash: String) -> Result<Transaction, lmdb::Error> {
    let env = create_or_open_env().unwrap();
    let db = open_database(&env, Some("Transactions")).unwrap();
    let txn = env.begin_ro_txn().unwrap();
    let key = tx_hash.as_bytes();
    let value = txn.get(db, &key).unwrap();
    let transaction = Transaction::from_bytes(value);
    Ok(transaction.unwrap())
}

pub fn get_number_tx() -> Result<u128, lmdb::Error> {
    // Attempt to create or open the environment, propagating any errors
    let env = create_or_open_env().unwrap();

    // Open the database, handling potential errors
    let db = open_database(&env, Some("Transactions"))?;

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

pub fn get_latest_tx() -> Result<Transaction, lmdb::Error> {
    let env = create_or_open_env().unwrap();
    let db = open_database(&env, Some("Transactions")).unwrap();
    let txn = env.begin_ro_txn()?;
    let mut cursor = txn.open_ro_cursor(db)?;
    let mut data = Vec::new();
    for (_key, value) in cursor.iter() {
        let value = Transaction::from_bytes(value);
        data.push(value.unwrap());
    }
    let last_tx = data.pop();
    Ok(last_tx.unwrap())
}
