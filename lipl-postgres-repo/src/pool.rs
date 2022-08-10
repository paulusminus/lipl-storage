use deadpool_postgres::{Pool, Manager};
use tokio_postgres::Config;
use tokio_postgres::tls::NoTls;

pub fn get(connection: &str, max_size: usize) -> Result<Pool, crate::error::PostgresRepoError> {
    connection.parse::<Config>()
    .map_err(crate::error::PostgresRepoError::from)
    .and_then(|config| 
        Pool::builder(
            Manager::from_config(config, NoTls, Default::default())
        )
        .max_size(max_size)
        .build()
        .map_err(crate::error::PostgresRepoError::from)
    )
}
