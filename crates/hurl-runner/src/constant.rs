pub const PREFIX_NAME: &str = "PREFIX";
pub const PREFIX_VALUE: &str = "http://localhost:3000/lipl/api/v1";
pub const OUTPUT_COLOR: bool = true;
pub const HURL_SCRIPTS: [&str; 3] = [
    include_str!("lipl-init.hurl"),
    include_str!("lipl-health.hurl"),
    include_str!("lipl.hurl"),
];
