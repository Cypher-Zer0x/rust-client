use crate::database::connection::{create_or_open_env, open_database};
use crate::database::read_blocks::get_last_block_number;
use crate::interface::Transaction;
use lmdb::{Cursor, Transaction as LmbdTransaction};

use super::read_blocks::get_block_by_number;

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
    println!("Attempting to get transaction by hash: {}", tx_hash);

    let env_result = create_or_open_env();

    match &env_result {
        Ok(env) => println!("Environment successfully opened."),
        Err(e) => println!("Failed to open environment: {:?}", e),
    }
    let env = env_result.unwrap(); // Assuming you handle unwrap better in production

    let db_result = open_database(&env, Some("Transactions"));
    match db_result {
        Ok(_) => println!("Database successfully opened."),
        Err(e) => println!("Failed to open database: {:?}", e),
    }
    let db = db_result?;

    let txn_result = env.begin_ro_txn();
    match txn_result {
        Ok(_) => println!("Read-only transaction successfully begun."),
        Err(e) => println!("Failed to begin read-only transaction: {:?}", e),
    }
    let txn = txn_result?;

    let key = tx_hash.as_bytes();
    match txn.get(db, &key) {
        Ok(value) => {
            println!("Transaction found for hash: {}", tx_hash);
            let transaction = Transaction::from_bytes(value);
            match transaction {
                Ok(t) => {
                    println!("Transaction deserialized successfully.");
                    Ok(t)
                },
                Err(e) => {
                    println!("Failed to deserialize transaction: {:?}", e);
                    Err(lmdb::Error::Other(12)) // Adjust according to your error handling
                }
            }
        },
        Err(e) => {
            if e == lmdb::Error::NotFound {
                println!("Transaction not found for hash: {}", tx_hash);
            } else {
                println!("Error retrieving transaction: {:?}", e);
            }
            Err(e)
        },
    }
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

pub fn get_latest_transactions() -> Result<Vec<Transaction>, lmdb::Error> {
    let last_block = get_last_block_number().unwrap(); // Assume you have a function to retrieve the latest block number from the "Blocks" database
    let mut txns = Vec::new();
    let mut i = 0;
    while (txns.len() < 10 || i < 100) {
        if last_block.unwrap() - i == 0 {
            break;
        }
        let block_number = last_block.unwrap() - i;
        // Get the transactions from the "Transactions" database for this block
        let block_transactions = match get_block_by_number(block_number) {
            Ok(txns) => txns,   // If there are transactions in the block, return them
            Err(_) => continue, // Otherwise, skip to the next iteration and check the previous block (if any)
        };
        txns.extend(block_transactions.transactions.into_iter());
        i += 1;
    }
    Ok(txns)
}
