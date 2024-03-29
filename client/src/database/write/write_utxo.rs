use crate::database::connection;
use crate::interface::UTXO;
use lmdb::Transaction as LmdbTransaction;
use lmdb::WriteFlags;

pub fn insert_utxo(utxo: UTXO) -> Result<(), lmdb::Error> {
    let env = connection::create_or_open_env().unwrap();
    let db = connection::open_database(&env, Some("UTXO"))?;
    let binding_env = env;
    let mut txn = binding_env.begin_rw_txn()?;
    txn.put(
        db,
        &utxo.get_hash().as_bytes(),
        &utxo.to_bytes(),
        WriteFlags::empty(),
    )?;
    txn.commit()?;
    // println!("UTXO written successfully.");
    Ok(())
}

// pub fn insert_payment_utxo(utxo: PaymentUTXO) -> Result<(), lmdb::Error> {
//     let env = connection::create_or_open_env().unwrap();
//     let db = connection::open_database(&env, Some("UTXO"))?;
//     let binding_env = env;
//     let mut txn = binding_env.begin_rw_txn()?;
//     txn.put(
//         db,
//         &utxo.get_hash().as_bytes(),
//         &utxo.to_bytes(),
//         WriteFlags::empty(),
//     )?;
//     txn.commit()?;
//     // println!("UTXO written successfully.");
//     Ok(())
// }

// remove an utxo from the database
pub fn remove_utxo(hash: String) -> Result<(), lmdb::Error> {
    let env = connection::create_or_open_env().unwrap();
    let db = connection::open_database(&env, Some("UTXO"))?;
    let binding_env = env;
    let mut txn = binding_env.begin_rw_txn()?;
    txn.del(db, &hash.as_bytes(), None)?;
    txn.commit()?;
    // println!("UTXO removed successfully.");
    Ok(())
}
