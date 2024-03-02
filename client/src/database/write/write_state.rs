use crate::database::connection;
use lmdb::Transaction as LmdbTransaction;
use lmdb::WriteFlags;

pub fn insert_last_state_proven(state_t_1: String) -> Result<(), lmdb::Error> {
    let env = connection::create_or_open_env().unwrap();
    let db = connection::open_database(&env, Some("State"))?;
    let binding_env = env;
    let mut txn = binding_env.begin_rw_txn()?;
    let key = "last_state_proven".as_bytes();
    let inputs_data = state_t_1.as_bytes();
    txn.put(db, &key, &inputs_data, WriteFlags::empty())?;
    txn.commit()?;
    Ok(())
}

pub fn insert_last_block_proven(block_number: String) -> Result<(), lmdb::Error> {
    let env = connection::create_or_open_env().unwrap();
    let db = connection::open_database(&env, Some("State"))?;
    let binding_env = env;
    let mut txn = binding_env.begin_rw_txn()?;
    let key = "last_block_proven".as_bytes();
    let inputs_data = block_number.as_bytes();
    txn.put(db, &key, &inputs_data, WriteFlags::empty())?;
    txn.commit()?;
    Ok(())
}
