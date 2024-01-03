use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub const PREFIX: &str = "/lipl/api/v1";
pub const DEFAULT_LOG_FILTER: &str = "info,tower_http=debug,tokio_postgres=warn";
pub const PG_CONNECTION: &str = "host=/run/postgresql dbname=test user=paul";
pub const RUST_LOG: &str = "RUST_LOG";
pub const USE_IPV6: bool = false;
pub const PORT: u16 = 3000;
pub const IPV4_LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
pub const IPV6_LOCALHOST: IpAddr = IpAddr::V6(Ipv6Addr::LOCALHOST);
pub const BASIC_AUTH_USERNAME: &str = "paul";
pub const BASIC_AUTH_PASSWORD: &str = "CumGranoSalis";
