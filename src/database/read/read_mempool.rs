use crate::database::connection::{create_or_open_env, open_database};
use crate::interface::PendingTransaction;
use lmdb::{Cursor, Transaction as LmbdTransaction};

// this function returns all the transactions in the mempool
pub fn get_mempool() -> Result<Vec<PendingTransaction>, lmdb::Error> {
    let env = create_or_open_env().unwrap();
    let db = open_database(&env, Some("Mempool")).unwrap();
    let txn = env.begin_ro_txn()?;
    let mut data = Vec::new();
    {
        let mut cursor = txn.open_ro_cursor(db)?;
        for (_key, value) in cursor.iter() {
            let value = PendingTransaction::from_bytes(value);
            data.push(value.unwrap());
        }
    }
    txn.commit()?;
    Ok(data)
}
