use deadpool_postgres::{Pool, Manager, ManagerConfig, RecyclingMethod};
use tokio_postgres::Config;
use tokio_postgres::tls::NoTls;

pub fn get(connection: &str, max_size: usize) -> anyhow::Result<Pool> {
    let postgres_config: Config = connection.parse()?;
    let manager_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast
    };

    let mgr = Manager::from_config(postgres_config, NoTls, manager_config);
    let pool = Pool::builder(mgr).max_size(max_size).build()?;
    Ok(pool)
}
