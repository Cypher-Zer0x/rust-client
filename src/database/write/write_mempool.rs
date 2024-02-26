use crate::database::connection;
use crate::interface::{PendingTransaction, PendingUserDepositTx, UserDepositEvent};
use lmdb::Transaction as LmdbTransaction;
use lmdb::WriteFlags;

// this function inserts a user deposit transaction into the mempool
pub fn insert_user_deposit_mempool(event: UserDepositEvent) -> Result<(), lmdb::Error> {
    let user_deposit_tx = PendingUserDepositTx::from_user_deposit_event(event);
    let tx = PendingTransaction::from_user_deposit_tx(user_deposit_tx.clone());
    let env = connection::create_or_open_env().unwrap();
    let db = connection::open_database(&env, Some("Mempool"))?;
    let binding_env = env;
    let mut txn = binding_env.begin_rw_txn()?;
    txn.put(
        db,
        &user_deposit_tx.hash.as_bytes(),
        &tx.to_bytes().unwrap(),
        WriteFlags::empty(),
    )?;
    txn.commit()?;
    println!("Deposit written successfully.");
    Ok(())
}

// this function deletes a transaction from the mempool
pub fn delete_transaction_from_mempool(hash: String) -> Result<(), lmdb::Error> {
    let env = connection::create_or_open_env().unwrap();
    let db = connection::open_database(&env, Some("Mempool"))?;
    let binding_env = env;
    let mut txn = binding_env.begin_rw_txn()?;
    txn.del(db, &hash.as_bytes(), None)?;
    txn.commit()?;
    println!("Transaction deleted successfully.");
    Ok(())
}

pub fn insert_transaction_in_mempool(tx: PendingTransaction) -> Result<(), lmdb::Error> {
    let env = connection::create_or_open_env().unwrap();
    let db = connection::open_database(&env, Some("Mempool"))?;
    let binding_env = env;
    let mut txn = binding_env.begin_rw_txn()?;
    txn.put(
        db,
        &tx.get_hash().as_bytes(),
        &tx.to_bytes().unwrap(),
        WriteFlags::empty(),
    )?;
    txn.commit()?;
    println!("Transaction written successfully.");
    Ok(())
}
