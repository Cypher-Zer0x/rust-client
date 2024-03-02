use crate::database::connection;
use crate::interface::{ringCTx, Transaction, UserDepositTx};
use lmdb::Transaction as LmdbTransaction;
use lmdb::WriteFlags;

pub fn insert_user_deposit_tx(user_deposit_tx: UserDepositTx) -> Result<(), lmdb::Error> {
    let tx = Transaction::from_user_deposit_tx(user_deposit_tx.clone());
    let env = connection::create_or_open_env().unwrap();
    let db = connection::open_database(&env, Some("Transactions"))?;
    let binding_env = env;
    let mut txn = binding_env.begin_rw_txn()?;
    txn.put(
        db,
        &user_deposit_tx.hash.as_bytes(),
        &tx.to_bytes(),
        WriteFlags::empty(),
    )?;
    txn.commit()?;

    // println!("Deposit written successfully.");

    Ok(())
}

pub fn insert_ring_ct(ring_ct: ringCTx) -> Result<(), lmdb::Error> {
    let env = connection::create_or_open_env().unwrap();
    let db = connection::open_database(&env, Some("Transactions"))?;
    let binding_env = env;
    let mut txn = binding_env.begin_rw_txn()?;
    txn.put(
        db,
        &ring_ct.hash.as_bytes(),
        &ring_ct.to_bytes().unwrap(),
        WriteFlags::empty(),
    )?;
    txn.commit()?;

    // println!("RingCT written successfully.");

    Ok(())
}

pub fn insert_transaction(tx: Transaction) -> Result<(), lmdb::Error> {
    let env = connection::create_or_open_env().unwrap();
    let db = connection::open_database(&env, Some("Transactions"))?;
    let binding_env = env;
    let mut txn = binding_env.begin_rw_txn()?;
    txn.put(
        db,
        &tx.get_hash().as_bytes(),
        &tx.to_bytes(),
        WriteFlags::empty(),
    )?;
    txn.commit()?;

    // println!("Transaction written successfully.");

    Ok(())
}
