use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Query {
    pub full: bool,
}
