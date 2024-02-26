use lmdb::{DatabaseFlags, Environment};
use std::{fs, io, path::Path};

/// Sets up the LMDB databases.
///
/// # Errors
///
/// Returns an error if the environment directory cannot be created, or if there
/// is a failure in setting up the LMDB environment or databases.
pub fn set_up_mldb() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("./lmdb_database");
    // More robust directory creation with error handling
    if !path.exists() {
        fs::create_dir_all(path).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to create LMDB environment directory: {}", e),
            )
        })?;
    }
    let env = Environment::new().set_max_dbs(7).open(path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to open LMDB environment: {}", e),
        )
    })?;
    // More robust database creation with error handling
    create_db(&env, "UTXO")?;
    create_db(&env, "Transactions")?;
    create_db(&env, "Blocks")?;
    create_db(&env, "Mempool")?;
    create_db(&env, "Index")?; // block indexes mapped to block hashes
    create_db(&env, "Validators")?;

    println!("Set up completed successfully.");
    Ok(())
}

/// Helper function to create a database within the given environment.
///
/// # Parameters
///
/// * `env` - Reference to the LMDB `Environment`.
/// * `name` - Name of the database to create.
///
/// # Errors
///
/// Returns an error if the database cannot be created.
fn create_db(env: &Environment, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    env.create_db(Some(name), DatabaseFlags::empty())
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to create {} database: {}", name, e),
            )
        })?;
    Ok(())
}
