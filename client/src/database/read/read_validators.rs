use crate::database::connection::{create_or_open_env, open_database};
use crate::interface::nodes::Validator;
use lmdb::{Cursor, Transaction as LmbdTransaction};

// this function returns all the validators from the database
pub fn get_validators() -> Result<Vec<Validator>, lmdb::Error> {
    let env = create_or_open_env().unwrap();
    let db = open_database(&env, Some("Validators")).unwrap();
    let txn = env.begin_ro_txn().unwrap();
    let mut cursor = txn.open_ro_cursor(db).unwrap();
    let mut validators: Vec<Validator> = Vec::new();
    for (key, value) in cursor.iter() {
        let validator = Validator::from_bytes(&value).unwrap();
        validators.push(validator);
    }
    return Ok(validators);
}
