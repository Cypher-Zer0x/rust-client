use lmdb::Database;
use lmdb::Environment;
use std::io;
use std::path::Path;

// this function creates or opens the LMDB environment
pub fn create_or_open_env() -> Result<Environment, std::io::Error> {
    let path = Path::new("./lmdb_database");
    let env = Environment::new().set_max_dbs(5).open(path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to open LMDB environment: {}", e),
        )
    });
    env
}

// this function opens the LMDB database
pub fn open_database(env: &Environment, name: Option<&str>) -> Result<Database, lmdb::Error> {
    let db = env.open_db(name)?;
    Ok(db)
}
