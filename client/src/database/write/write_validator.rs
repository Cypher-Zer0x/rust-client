use crate::database::connection;
use crate::interface::nodes::Validator;
use lmdb::Transaction as LmdbTransaction;
use lmdb::WriteFlags;

// this function inserts a new validator into the database
pub fn insert_validator(validators: Vec<Validator>) -> Result<(), lmdb::Error> {
    // todo: a priori marche pas
    let env = connection::create_or_open_env().unwrap();
    let db = connection::open_database(&env, Some("Validators"))?;
    let binding_env = env;
    let mut txn = binding_env.begin_rw_txn()?;
    for validator in validators {
        txn.put(
            db,
            &validator.pubkey.as_bytes(),
            &validator.to_bytes().unwrap(),
            WriteFlags::empty(),
        )?;
        println!(
            "New Validator detected: {:?}",
            validator.node.ip + ":" + &validator.node.port
        );
    }

    txn.commit()?;

    Ok(())
}
