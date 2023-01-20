use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::tls::NoTls;

pub fn get(connection: &str) -> Result<PostgresConnectionManager<NoTls>, crate::PostgresRepoError> {
    let manager = PostgresConnectionManager::new_from_stringlike(connection, NoTls)?;
    Ok(manager)   
}
