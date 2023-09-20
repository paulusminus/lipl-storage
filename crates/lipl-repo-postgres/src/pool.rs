use bb8_postgres::PostgresConnectionManager;
use bb8_postgres::tokio_postgres::tls::NoTls;
use lipl_core::{postgres_error, Result};

pub fn get(connection: &str) -> Result<PostgresConnectionManager<NoTls>> {
    let manager = 
        PostgresConnectionManager::new_from_stringlike(connection, NoTls)
        .map_err(postgres_error)?;
    Ok(manager)   
}
