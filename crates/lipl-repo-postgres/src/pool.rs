use bb8_postgres::PostgresConnectionManager;
use bb8_postgres::tokio_postgres::tls::NoTls;

pub fn get(connection: &str) -> Result<PostgresConnectionManager<NoTls>, crate::Error> {
    let manager = 
        PostgresConnectionManager::new_from_stringlike(connection, NoTls)
        .map_err(|error| crate::Error::Postgres(Box::new(error)))?;
    Ok(manager)   
}
