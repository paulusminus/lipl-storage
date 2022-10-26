pub const PREFIX: &str = "/api/v1";
pub const DEFAULT_LOG_FILTER: &str = "info,tower_http=debug,tokio_postgres=warn";
pub const PG_CONNECTION: &str = "host=/run/postgresql dbname=test user=paul";
pub const RUST_LOG: &str = "RUST_LOG";
pub const LOCALHOST: [u8; 4] = [127, 0, 0, 1];
pub const PORT: u16 = 3000;
